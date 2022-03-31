use anyhow::Result;
use ring::rand::SecureRandom;
use ring::{digest::SHA256_OUTPUT_LEN, hmac, rand};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use recesser_core::encoding::base64;
use recesser_core::hash::DIGEST_LEN;
use recesser_core::user::{Scope, User};

use crate::error::UserError;

#[derive(Clone)]
pub struct HmacKey(hmac::Key);

impl HmacKey {
    pub fn new(key_value: &[u8; SHA256_OUTPUT_LEN]) -> Self {
        Self(hmac::Key::new(hmac::HMAC_SHA256, key_value))
    }

    pub fn generate_key_value(rng: &dyn SecureRandom) -> Result<[u8; SHA256_OUTPUT_LEN]> {
        Ok(rand::generate(rng)?.expose())
    }

    pub fn key(&self) -> &hmac::Key {
        &self.0
    }
}

pub struct Token {
    header: Header,
    claims: Claims,
    mac: Mac,
}

#[derive(Deserialize, Serialize)]
struct Header {
    alg: Algorithm,
    typ: Type,
}

#[derive(Deserialize, Serialize)]
enum Algorithm {
    HS256,
}

#[derive(Deserialize, Serialize)]
enum Type {
    #[serde(rename = "JWT")]
    Jwt,
}

#[derive(Deserialize, Serialize)]
struct Claims {
    id: String,
    scope: Scope,
}

struct Mac([u8; DIGEST_LEN]);

impl Token {
    pub fn create(scope: Scope, key: &HmacKey) -> Result<Self> {
        let header = Header::new();
        let claims = Claims::new(scope);
        let mac = Mac::calculate(key, payload(&header, &claims)?.as_bytes())?;
        Ok(Self {
            header,
            claims,
            mac,
        })
    }

    pub fn validate(input: &str, key: &HmacKey) -> Result<Self> {
        let token = Token::from_string(input)?;
        let extracted_mac = &token.mac;
        extracted_mac.verify(key, payload(&token.header, &token.claims)?.as_bytes())?;
        Ok(token)
    }

    pub fn validate_scope(&self, expected_scope: Scope) -> std::result::Result<(), UserError> {
        let authenticated = match expected_scope {
            Scope::User => match self.claims.scope {
                Scope::User => true,
                Scope::Machine => true,
                Scope::Admin => true,
            },
            Scope::Machine => match self.claims.scope {
                Scope::User => false,
                Scope::Machine => true,
                Scope::Admin => true,
            },
            Scope::Admin => match self.claims.scope {
                Scope::User => false,
                Scope::Machine => false,
                Scope::Admin => true,
            },
        };

        if !authenticated {
            return Err(UserError::Unauthorized);
        }

        Ok(())
    }

    pub fn to_string(&self) -> Result<String> {
        Ok(format!(
            "{}.{}.{}",
            self.header.to_base64()?,
            self.claims.to_base64()?,
            self.mac.to_base64()
        ))
    }

    pub fn extract_user(&self) -> User {
        User {
            id: String::from(&self.claims.id),
            scope: self.claims.scope.clone(),
        }
    }

    fn from_string(input: &str) -> Result<Self> {
        let mut split_input = input.split('.');

        let header = match split_input.next() {
            Some(s) => Header::from_base64(s)?,
            None => anyhow::bail!("Failed to deserialize token header"),
        };

        let claims = match split_input.next() {
            Some(s) => Claims::from_base64(s)?,
            None => anyhow::bail!("Failed to deserialize token claims"),
        };

        let mac = match split_input.next() {
            Some(s) => Mac::from_base64(s)?,
            None => anyhow::bail!("Failed to deserialize token mac"),
        };

        Ok(Self {
            header,
            claims,
            mac,
        })
    }
}

fn payload(header: &Header, claims: &Claims) -> Result<String> {
    Ok(format!("{}.{}", header.to_base64()?, claims.to_base64()?))
}

impl Header {
    fn new() -> Self {
        Self {
            alg: Algorithm::HS256,
            typ: Type::Jwt,
        }
    }
}

impl Claims {
    fn new(scope: Scope) -> Self {
        let uuid = Uuid::new_v4();
        let mut buf = Uuid::encode_buffer();
        let encoded_uuid = uuid.to_hyphenated().encode_lower(&mut buf);
        Self {
            id: String::from(encoded_uuid),
            scope,
        }
    }
}

impl Mac {
    fn calculate(key: &HmacKey, payload: &[u8]) -> Result<Mac> {
        let keyed_hash = hmac::sign(key.key(), payload);
        Ok(Mac(keyed_hash.as_ref().try_into()?))
    }

    fn verify(&self, key: &HmacKey, payload: &[u8]) -> Result<()> {
        hmac::verify(key.key(), payload, &self.0)?;
        Ok(())
    }

    fn to_base64(&self) -> String {
        let mut buf = String::with_capacity(46);
        base64::encode_into_buf(&self.0, &mut buf);
        buf
    }

    fn from_base64(input: &str) -> Result<Self> {
        let mut buf = [0; DIGEST_LEN];
        base64::decode_into_slice(input, &mut buf)?;
        Ok(Self(buf))
    }
}

trait ToBase64 {
    fn to_base64(&self) -> Result<String>
    where
        Self: Serialize + Sized,
    {
        let b = serde_json::to_vec(self)?;
        Ok(base64::encode(&b))
    }

    fn from_base64(input: &str) -> Result<Self>
    where
        Self: DeserializeOwned,
    {
        let b = base64::decode(input)?;
        Ok(serde_json::from_slice(&b)?)
    }
}

impl ToBase64 for Header {}

impl ToBase64 for Claims {}

pub mod middleware {
    use actix_web::dev::ServiceRequest;
    use actix_web::Error;
    use actix_web::{web, HttpMessage};
    use actix_web_httpauth::extractors::bearer::BearerAuth;

    use crate::error::UserError;
    use crate::AppState;

    use super::{Scope, Token};

    pub fn validate_scope(req: &impl HttpMessage, scope: Scope) -> Result<(), UserError> {
        let ext = req.extensions();
        let token = ext.get::<Token>().ok_or(UserError::Internal)?;
        token.validate_scope(scope)
    }

    pub async fn validator(
        req: ServiceRequest,
        credentials: BearerAuth,
    ) -> Result<ServiceRequest, Error> {
        let app_state = extract_app_state(&req)?;
        let token = validate_token(credentials, app_state)?;
        req.extensions_mut().insert(token);
        Ok(req)
    }

    fn extract_app_state(req: &ServiceRequest) -> Result<&web::Data<AppState>, UserError> {
        req.app_data::<web::Data<AppState>>()
            .ok_or(UserError::Internal)
    }

    fn validate_token(
        credentials: BearerAuth,
        app_state: &web::Data<AppState>,
    ) -> Result<Token, UserError> {
        let token_str = credentials.token();
        let hmac_key = app_state.hmac_key.lock().unwrap();
        Token::validate(token_str, &hmac_key).map_err(UserError::unauthorized)
    }
}

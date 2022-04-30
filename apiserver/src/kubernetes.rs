use std::collections::BTreeMap;

use anyhow::Result;
use k8s_openapi::api::core::v1::Secret;
use kube::api::{Api, ObjectMeta, PostParams};
use recesser_core::repository::KeyPair;

use crate::auth::Token;

pub struct KubernetesApiserver {
    recesser_secrets: Api<Secret>,
    argo_secrets: Api<Secret>,
}

impl KubernetesApiserver {
    pub async fn new() -> Result<Self> {
        let client = kube::Client::try_default().await?;
        let recesser_secrets: Api<Secret> = Api::namespaced(client.clone(), "recesser");
        let argo_secrets: Api<Secret> = Api::namespaced(client, "argo");
        tracing::info!("Connected to kubernetes apiserver");
        Ok(Self {
            recesser_secrets,
            argo_secrets,
        })
    }

    pub async fn create_ssh_secret(&self, key_pair: &KeyPair) -> Result<()> {
        let secret = Secret {
            metadata: ObjectMeta {
                name: Some(key_pair.public_key.fingerprint.to_string()),
                ..Default::default()
            },
            string_data: Some(BTreeMap::from([(
                "ssh-privatekey".into(),
                key_pair.private_key.to_string(),
            )])),
            type_: Some("kubernetes.io/ssh-auth".into()),
            ..Default::default()
        };
        self.create_argo_secret(&secret).await?;
        Ok(())
    }

    pub async fn create_token_secret(&self, name: &str, token: &Token) -> Result<()> {
        let secret = Secret {
            metadata: ObjectMeta {
                name: Some(name.into()),
                ..Default::default()
            },
            string_data: Some(BTreeMap::from([("token".into(), token.to_string()?)])),
            ..Default::default()
        };
        self.create_recesser_secret(&secret).await?;
        Ok(())
    }

    async fn create_recesser_secret(&self, secret: &Secret) -> Result<()> {
        let params = PostParams::default();
        self.recesser_secrets.create(&params, secret).await?;
        Ok(())
    }

    async fn create_argo_secret(&self, secret: &Secret) -> Result<()> {
        let params = PostParams::default();
        self.argo_secrets.create(&params, secret).await?;
        Ok(())
    }
}

use std::collections::BTreeMap;

use anyhow::Result;
use k8s_openapi::api::core::v1::Secret;
use kube::api::{Api, ObjectMeta, PostParams};

pub struct KubernetesApiserver {
    recesser_secrets: Api<Secret>,
    argo_secrets: Api<Secret>,
}

impl KubernetesApiserver {
    pub async fn new() -> Result<Self> {
        let client = kube::Client::try_default().await?;
        let recesser_secrets: Api<Secret> = Api::namespaced(client.clone(), "recesser");
        let argo_secrets: Api<Secret> = Api::namespaced(client, "argo");
        Ok(Self {
            recesser_secrets,
            argo_secrets,
        })
    }

    pub async fn create_recesser_secret(&self, secret: &Secret) -> Result<()> {
        let params = PostParams::default();
        self.recesser_secrets.create(&params, secret).await?;
        Ok(())
    }

    pub async fn create_argo_secret(&self, secret: &Secret) -> Result<()> {
        let params = PostParams::default();
        self.argo_secrets.create(&params, secret).await?;
        Ok(())
    }
}

pub fn create_ssh_secret(name: String, private_key: String) -> Result<Secret> {
    let secret = Secret {
        metadata: ObjectMeta {
            name: Some(name),
            ..Default::default()
        },
        string_data: Some(BTreeMap::from([("ssh-privatekey".into(), private_key)])),
        type_: Some("kubernetes.io/ssh-auth".into()),
        ..Default::default()
    };
    Ok(secret)
}

pub fn create_token_secret(name: String, token: String) -> Result<Secret> {
    let secret = Secret {
        metadata: ObjectMeta {
            name: Some(name),
            ..Default::default()
        },
        string_data: Some(BTreeMap::from([("token".into(), token)])),
        ..Default::default()
    };
    Ok(secret)
}

pub mod argo_workflows;
mod template;

use anyhow::Result;
use k8s_openapi::api::core::v1::Secret;
use kube::api::{Api, PostParams};
use serde::{Deserialize, Serialize};

use template::{construct_from_template, Template};

const ARGO_NAMESPACE: &str = "argo";

pub struct KubernetesApiserver {
    secrets: Api<Secret>,
}

impl KubernetesApiserver {
    pub async fn new() -> Result<Self> {
        let client = kube::Client::try_default().await?;
        let secrets: Api<Secret> = Api::namespaced(client, ARGO_NAMESPACE);
        Ok(Self { secrets })
    }

    pub async fn create_secret(&self, ssh_secret: &SSHSecret) -> Result<()> {
        self.secrets
            .create(&PostParams::default(), &ssh_secret.0)
            .await?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct SSHSecret(Secret);

impl SSHSecret {
    pub fn new(name: String, private_key: String) -> Result<Self> {
        let ssh_private_key = construct_from_template(
            Template::SSHPrivateKey,
            minijinja::context!(name, private_key),
        )?;
        Ok(ssh_private_key)
    }
}

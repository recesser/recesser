pub use server::ArgoWorkflowsServer;
pub use ssh_secret::SSHSecret;
pub use workflow::Workflow;

mod server {
    use anyhow::Result;
    use reqwest::Client;

    use super::Workflow;

    #[derive(Clone)]
    pub struct ArgoWorkflowsServer {
        addr: String,
        client: Client,
    }

    impl ArgoWorkflowsServer {
        pub fn new(addr: &str) -> Self {
            Self {
                addr: String::from(addr),
                client: reqwest::Client::new(),
            }
        }

        pub async fn submit(&self, workflow: Workflow) -> Result<()> {
            self.client
                .post(format!("http://{}/api/v1/workflows/argo/submit", self.addr))
                .json(&workflow)
                .send()
                .await?;
            Ok(())
        }
    }
}

/// Kubernetes SSH secret
mod ssh_secret {
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SSHSecret {
        api_version: String,
        kind: String,
        metadata: SSHSecretMetadata,
        r#type: String,
        data: SSHSecretData,
    }

    #[derive(Serialize, Debug)]
    struct SSHSecretMetadata {
        name: String,
    }

    #[derive(Serialize, Debug)]
    struct SSHSecretData {
        #[serde(rename = "ssh-privatekey")]
        ssh_privatekey: String,
    }

    impl SSHSecret {
        pub fn new(name: String, private_key: String) -> Self {
            Self {
                api_version: "v1".into(),
                kind: "Secret".into(),
                metadata: SSHSecretMetadata { name },
                r#type: "kubernetes.io/ssh-auth".into(),
                data: SSHSecretData {
                    ssh_privatekey: private_key,
                },
            }
        }
    }
}

mod workflow {
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Workflow {
        api_version: String,
        kind: String,
        spec: WorkflowSpec,
    }

    #[derive(Serialize, Debug)]
    struct WorkflowSpec {
        entrypoint: String,
        templates: Vec<Template>,
    }

    #[derive(Serialize, Debug)]
    struct Template {
        name: String,
        inputs: Option<Inputs>,
        container: Option<Container>,
        dag: Option<DagTemplate>,
    }

    #[derive(Serialize, Debug)]
    struct Inputs {
        artifacts: Vec<Artifact>,
    }

    #[derive(Serialize, Debug)]
    struct Artifact {
        name: String,
        path: String,
        git: Option<GitArtifact>,
        http: Option<HTTPArtifact>,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct GitArtifact {
        repo: String,
        revision: String,
        ssh_private_key_secret: SSHPrivateKeySecret,
        depth: u32,
    }

    #[derive(Serialize, Debug)]
    struct SSHPrivateKeySecret {
        name: String,
        key: String,
    }

    #[derive(Serialize, Debug)]
    struct HTTPArtifact {
        headers: Vec<Header>,
        url: String,
    }

    #[derive(Serialize, Debug)]
    struct Header {
        name: String,
        value: String,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Container {
        image: String,
        command: String,
        args: Vec<String>,
        working_dir: String,
    }

    #[derive(Serialize, Debug)]
    struct DagTemplate {
        tasks: Vec<DagTask>,
    }

    #[derive(Serialize, Debug)]
    struct DagTask {
        name: String,
        dependencies: Vec<String>,
        template: String,
    }
}

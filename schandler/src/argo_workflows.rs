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
    use anyhow::Result;
    use recesser_core::repository::Repository;
    use serde::Serialize;

    use crate::pipeline::{Kind, Pipeline};

    const CLI_IMAGE: &str = "recesser/cli:0.1.0";

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Workflow {
        api_version: String,
        kind: String,
        spec: WorkflowSpec,
    }

    impl Workflow {
        pub fn from_pipeline(
            pipeline: Pipeline,
            repository: Repository,
            ssh_secret_name: String,
        ) -> Result<Self> {
            let pipeline = match pipeline.kind {
                Kind::TemplatePipeline(template_pipeline) => template_pipeline,
                _ => return Err(anyhow::anyhow!("CustomTemplate is not yet implemented")),
            };

            let mut templates = Vec::new();
            let mut workflow_steps = Vec::new();

            if let Some(inputs) = pipeline.inputs {
                let download_artifacts_container = Container {
                    image: CLI_IMAGE.into(),
                    command: vec![format!("rcssr download {}", inputs.join(""))],
                    args: None,
                    working_dir: None,
                };
                let download_artifacts_template = Template {
                    name: "download_artifacts".into(),
                    inputs: None,
                    container: Some(download_artifacts_container),
                    steps: None,
                };
                templates.push(download_artifacts_template);

                let download_artifacts_step = WorkflowStep {
                    name: "Download Artifacts Step".into(),
                    template: "download_artifacts".into(),
                };
                workflow_steps.push(vec![download_artifacts_step]);
            }

            let git_artifact = GitArtifact {
                repo: repository.url,
                revision: repository.last_commit.to_string(),
                ssh_private_key_secret: SSHPrivateKeySecret::new(ssh_secret_name),
                depth: 0,
            };
            let main_template_inputs = Inputs {
                artifacts: vec![Artifact {
                    name: "source-code".into(),
                    path: None,
                    git: Some(git_artifact),
                }],
            };
            let main_container = Container {
                image: format!("{}:{}", pipeline.template.name, pipeline.template.version),
                command: pipeline.entrypoint,
                args: None,
                working_dir: None,
            };
            let main_template = Template {
                name: "main".into(),
                inputs: Some(main_template_inputs),
                container: Some(main_container),
                steps: None,
            };
            templates.push(main_template);

            let main_step = WorkflowStep {
                name: "Main Step".into(),
                template: "main".into(),
            };
            workflow_steps.push(vec![main_step]);

            let steps_template = Template {
                name: "steps".into(),
                inputs: None,
                container: None,
                steps: Some(workflow_steps),
            };
            templates.push(steps_template);

            let spec = WorkflowSpec {
                entrypoint: "steps".into(),
                templates,
            };
            Ok(Self::new(spec))
        }

        fn new(spec: WorkflowSpec) -> Self {
            Self {
                api_version: "argoproj.io/v1alpha1".into(),
                kind: "Workflow".into(),
                spec,
            }
        }
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
        steps: Option<Vec<Vec<WorkflowStep>>>,
    }

    #[derive(Serialize, Debug)]
    struct Inputs {
        artifacts: Vec<Artifact>,
    }

    #[derive(Serialize, Debug)]
    struct Artifact {
        name: String,
        path: Option<String>,
        git: Option<GitArtifact>,
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

    impl SSHPrivateKeySecret {
        pub fn new(name: String) -> Self {
            Self {
                name,
                key: "ssh-private-key".into(),
            }
        }
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Container {
        image: String,
        command: Vec<String>,
        args: Option<Vec<String>>,
        working_dir: Option<String>,
    }

    #[derive(Serialize, Debug)]
    struct WorkflowStep {
        name: String,
        template: String,
    }
}

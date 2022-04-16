pub use server::ArgoWorkflowsServer;
pub use ssh_secret::SSHSecret;
pub use workflow::Workflow;

/// HTTP client for Argo Workflows server
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
        api_version: &'static str,
        kind: &'static str,
        metadata: SSHSecretMetadata,
        r#type: &'static str,
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
                api_version: "v1",
                kind: "Secret",
                metadata: SSHSecretMetadata { name },
                r#type: "kubernetes.io/ssh-auth",
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

    use crate::pipeline::{Kind, Pipeline, TemplatePipeline};

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Workflow {
        api_version: &'static str,
        kind: &'static str,
        metadata: Metadata,
        spec: WorkflowSpec,
    }

    #[derive(Serialize, Debug)]
    struct Metadata {
        name: String,
    }

    #[derive(Serialize, Debug)]
    struct WorkflowSpec {
        entrypoint: &'static str,
        templates: Vec<Template>,
    }

    #[serde_with::skip_serializing_none]
    #[derive(Serialize, Debug)]
    struct Template {
        name: &'static str,
        inputs: Option<Inputs>,
        container: Option<Container>,
        steps: Option<Vec<Vec<WorkflowStep>>>,
    }

    #[derive(Serialize, Debug)]
    struct Inputs {
        artifacts: Vec<Artifact>,
    }

    #[serde_with::skip_serializing_none]
    #[derive(Serialize, Debug)]
    struct Artifact {
        name: &'static str,
        path: Option<String>,
        git: Option<GitArtifact>,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct GitArtifact {
        repo: String,
        revision: &'static str,
        ssh_private_key_secret: SSHPrivateKeySecret,
        depth: u32,
    }

    #[derive(Serialize, Debug)]
    struct SSHPrivateKeySecret {
        name: String,
        key: &'static str,
    }

    #[serde_with::skip_serializing_none]
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
        name: &'static str,
        template: &'static str,
    }

    impl Workflow {
        pub fn from_pipeline(pipeline: Pipeline, repository: Repository) -> Result<Self> {
            let pipeline = match pipeline.kind {
                Kind::TemplatePipeline(template_pipeline) => template_pipeline,
                _ => return Err(anyhow::anyhow!("CustomTemplate is not yet implemented")),
            };

            let mut templates = Vec::new();
            let mut workflow_steps = Vec::new();

            // Add inputs step if artifacts are specified in pipeline
            if let Some(inputs) = &pipeline.inputs {
                templates.push(download_artifacts_template(inputs));
                workflow_steps.push(download_artifacts_step());
            }

            // Add main template that executes code
            templates.push(main_template(&repository, &pipeline));
            workflow_steps.push(main_step());

            // Add execution order template
            templates.push(steps_template(workflow_steps));

            let spec = WorkflowSpec::new(templates);
            Ok(Self::new(repository.name, spec))
        }

        fn new(name: String, spec: WorkflowSpec) -> Self {
            Self {
                api_version: "argoproj.io/v1alpha1",
                kind: "Workflow",
                metadata: Metadata { name },
                spec,
            }
        }
    }

    fn download_artifacts_template(inputs: &[String]) -> Template {
        let download_artifacts_container = Container {
            image: "recesser/cli:0.1.0".into(),
            command: vec![format!("rcssr download {}", inputs.join(" "))],
            args: None,
            working_dir: None,
        };
        Template {
            name: "download_artifacts",
            inputs: None,
            container: Some(download_artifacts_container),
            steps: None,
        }
    }

    fn download_artifacts_step() -> Vec<WorkflowStep> {
        vec![WorkflowStep {
            name: "Download Artifacts Step",
            template: "download_artifacts",
        }]
    }

    fn main_template_inputs(repository: &Repository) -> Inputs {
        let git_artifact = GitArtifact {
            repo: repository.url.clone(),
            revision: "HEAD",
            ssh_private_key_secret: SSHPrivateKeySecret::new(
                repository.public_key.fingerprint.to_string(),
            ),
            depth: 0,
        };
        Inputs {
            artifacts: vec![Artifact {
                name: "source-code",
                path: None,
                git: Some(git_artifact),
            }],
        }
    }

    fn main_template(repository: &Repository, pipeline: &TemplatePipeline) -> Template {
        let main_container = Container {
            image: format!(
                "recesser/{}-template:{}",
                pipeline.template.name, pipeline.template.version
            ),
            command: pipeline.entrypoint.clone(),
            args: None,
            working_dir: None,
        };
        Template {
            name: "main",
            inputs: Some(main_template_inputs(repository)),
            container: Some(main_container),
            steps: None,
        }
    }

    fn main_step() -> Vec<WorkflowStep> {
        vec![WorkflowStep {
            name: "Main Step",
            template: "main",
        }]
    }

    fn steps_template(workflow_steps: Vec<Vec<WorkflowStep>>) -> Template {
        Template {
            name: "steps",
            inputs: None,
            container: None,
            steps: Some(workflow_steps),
        }
    }

    impl WorkflowSpec {
        fn new(templates: Vec<Template>) -> Self {
            Self {
                entrypoint: "steps",
                templates,
            }
        }
    }

    impl SSHPrivateKeySecret {
        pub fn new(name: String) -> Self {
            Self {
                name,
                key: "ssh-private-key",
            }
        }
    }
}

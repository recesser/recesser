use std::fmt;

use anyhow::Result;

pub use server::ArgoWorkflowsServer;
pub use ssh_secret::SSHSecret;
pub use workflow::Workflow;

lazy_static::lazy_static! {
    static ref TEMPLATES: minijinja::Environment<'static> = {
        let mut env = minijinja::Environment::new();

        let template_workflow = include_str!("templates/template_workflow.yml.j2");
        env.add_template("template_workflow", template_workflow).unwrap();
        let template_workflow = include_str!("templates/ssh_private_key.yml.j2");
        env.add_template("ssh_private_key", template_workflow).unwrap();

        env
    };
}

enum Template {
    TemplateWorkflow,
    SSHPrivateKey,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Template::TemplateWorkflow => "template_workflow",
            Template::SSHPrivateKey => "ssh_private_key",
        };
        write!(f, "{s}")
    }
}

fn construct_from_template<S, D>(template: Template, ctx: S) -> Result<D>
where
    S: serde::Serialize,
    D: serde::de::DeserializeOwned,
{
    let template = TEMPLATES.get_template(&template.to_string())?;
    let rendered_template = template.render(ctx)?;
    Ok(serde_yaml::from_str(&rendered_template)?)
}

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

        pub async fn submit(&self, workflow: &Workflow) -> Result<()> {
            self.client
                .post(format!("http://{}/api/v1/workflows/argo/submit", self.addr))
                .json(workflow)
                .send()
                .await?;
            Ok(())
        }
    }
}

/// Kubernetes SSH secret
mod ssh_secret {
    use anyhow::Result;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(transparent)]
    pub struct SSHSecret(serde_json::Value);

    impl SSHSecret {
        pub fn new(name: String, private_key: String) -> Result<Self> {
            let ssh_private_key = super::construct_from_template(
                super::Template::SSHPrivateKey,
                minijinja::context!(name, private_key),
            )?;
            Ok(ssh_private_key)
        }
    }
}

mod workflow {
    use anyhow::Result;
    use recesser_core::repository::Repository;
    use serde::{Deserialize, Serialize};

    use crate::pipeline::{Kind, Pipeline};

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(transparent)]
    pub struct Workflow(serde_json::Value);

    impl Workflow {
        pub fn from_pipeline(pipeline: Pipeline, repository: Repository) -> Result<Self> {
            let metadata = pipeline.metadata;
            let workflow = match pipeline.kind {
                Kind::TemplatePipeline(pipeline) => super::construct_from_template(
                    super::Template::TemplateWorkflow,
                    minijinja::context!(metadata, pipeline, repository),
                )?,
                _ => return Err(anyhow::anyhow!("CustomTemplate is not yet implemented")),
            };

            Ok(workflow)
        }
    }
}

use std::fmt;

use anyhow::Result;

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

pub enum Template {
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

pub fn construct_from_template<S, D>(template: Template, ctx: S) -> Result<D>
where
    S: serde::Serialize,
    D: serde::de::DeserializeOwned,
{
    let template = TEMPLATES.get_template(&template.to_string())?;
    let rendered_template = template.render(ctx)?;
    Ok(serde_yaml::from_str(&rendered_template)?)
}

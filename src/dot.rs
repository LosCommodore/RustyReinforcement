use crate::{agent::QIterAgent, environment::Environment};
use anyhow::{Context, Result};
use minijinja::context;

const TEMPLATE_NAME: &str = "graph_dot.j2";

pub struct DotRenderer {
    env: minijinja::Environment<'static>, // not `pub`, so only this module can see it
}

impl DotRenderer {
    pub fn new() -> Result<Self> {
        let mut env = minijinja::Environment::new();

        env.add_template(TEMPLATE_NAME, include_str!("../templates/graph.dot.j2"))
            .context("template not found")?;

        Ok(Self { env })
    }

    pub fn render_agent<E: Environment>(&self, agent: QIterAgent<E>) -> String {
        let tr = agent.rewards();

        let nodes: Vec<minijinja::Value> = tr
            .into_iter()
            .map(|(k, v)| context! { id => k, score => v })
            .collect();

        let template = self.env.get_template(TEMPLATE_NAME).unwrap();

        template.render(context! { name => "World" }).unwrap()
    }
}

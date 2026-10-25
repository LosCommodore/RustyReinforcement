use std::collections::HashSet;

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

    pub fn render_agent<E: Environment>(&self, agent: QIterAgent<E>) -> String
    where
        E::O: Into<minijinja::Value>,
        E::R: Into<minijinja::Value>,
    {
        let tr = agent.rewards();

        let n: HashSet<_> = tr.keys().into_iter().map(|x| x.source).collect();

        /*
        let nodes: Vec<minijinja::Value> = tr
            .into_iter()
            .map(|(k, _v)| context! { id => k.dest.into(), score => 53 })
            .collect();
        */

        let nodes: Vec<minijinja::Value> = n
            .into_iter()
            .map(|x| context! { id => x.into(), score => 53 })
            .collect();

        let template = self.env.get_template(TEMPLATE_NAME).unwrap();

        template.render(nodes).unwrap()
    }
}

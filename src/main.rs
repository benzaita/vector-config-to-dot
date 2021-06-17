mod graph_emitter;

use crate::graph_emitter::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::From;
use std::io::{self, Read};

#[derive(Deserialize)]
struct VectorConfig {
    sinks: HashMap<String, ComponentConfig>,
    sources: HashMap<String, ComponentConfig>,
    transforms: HashMap<String, ComponentConfig>,
}

#[derive(Deserialize)]
struct ComponentConfig {
    #[serde(rename = "type")]
    component_type: String,
    inputs: Option<Vec<String>>,
}

enum InputName {
    Regular(String),
    Compound(String, String),
}
impl From<&String> for InputName {
    fn from(name: &String) -> Self {
        match name.find(".") {
            None => InputName::Regular(name.to_owned()),
            Some(dot_index) => InputName::Compound(
                name[0..dot_index].to_owned(),
                name[dot_index + 1..].to_owned(),
            ),
        }
    }
}

fn main() -> io::Result<()> {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf)?;

    let config: VectorConfig = toml::from_str(&buf)?;

    emit_graph_start(&vec![
        Attr::new("nodesep", "0.5".to_string()),
    ]);

    emit_cluster_start(
        "sources",
        &vec![
            Attr::new("color", "white".to_string()),
        ],
    );
    for (component_name, component_config) in &config.sources {
        emit_source_node(&component_name, &component_config);
    }
    emit_cluster_end();
    emit_cluster_start("transforms", &vec![
        Attr::new("color", "white".to_string()),
        ]);
    for (component_name, component_config) in &config.transforms {
        emit_transform_node(&component_name, &component_config);
    }
    emit_cluster_end();
    emit_cluster_start("sinks", &vec![
        Attr::new("color", "white".to_string(),
    )]);
    for (component_name, component_config) in &config.sinks {
        emit_sink_node(&component_name, &component_config);
    }
    emit_cluster_end();

    let mut all_components: HashMap<String, ComponentConfig> = HashMap::new();
    all_components.extend(config.sources.into_iter());
    all_components.extend(config.transforms.into_iter());
    all_components.extend(config.sinks.into_iter());
    for (component_name, component_config) in &all_components {
        emit_input_edges(&component_name, &component_config);
    }

    emit_graph_end();

    Ok(())
}

fn emit_input_edges(dest_component_name: &String, config: &ComponentConfig) {
    if let Some(inputs) = &config.inputs {
        for input_name in inputs {
            match InputName::from(input_name) {
                InputName::Regular(input_name) => {
                    emit_edge(&input_name, dest_component_name, &Vec::new())
                }
                InputName::Compound(input_name, subinput_name) => emit_edge(
                    &input_name,
                    dest_component_name,
                    &vec![Attr::new("label", subinput_name)],
                ),
            }
        }
    }
}

fn emit_source_node(name: &String, config: &ComponentConfig) {
    let attrs = vec![
        Attr::new("label", format!("{}\\n({})", name, config.component_type)),
        Attr::new("shape", "ellipse".to_string()),
        Attr::new("style", "rounded, filled".to_string()),
        Attr::new("colorscheme", "prgn4".to_string()),
        Attr::new("color", "3".to_string()),
    ];

    emit_node(&name, &attrs);
}

fn emit_transform_node(name: &String, config: &ComponentConfig) {
    let attrs = vec![
        Attr::new("label", format!("{}\\n({})", name, config.component_type)),
        Attr::new("shape", "box".to_string()),
        Attr::new("style", "rounded, filled".to_string()),
        Attr::new("colorscheme", "prgn4".to_string()),
        Attr::new("color", "2".to_string()),
    ];

    emit_node(&name, &attrs);
}

fn emit_sink_node(name: &String, config: &ComponentConfig) {
    let attrs = vec![
        Attr::new("label", format!("{}\\n({})", name, config.component_type)),
        Attr::new("shape", "ellipse".to_string()),
        Attr::new("style", "rounded, filled".to_string()),
        Attr::new("colorscheme", "prgn4".to_string()),
        Attr::new("color", "4".to_string()),
    ];

    emit_node(&name, &attrs);
}

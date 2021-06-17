pub struct Attr {
    name: String,
    value: String,
}

impl Attr {
    pub fn new(name: &str, value: String) -> Self {
        Attr {
            name: name.to_string(),
            value,
        }
    }
}

impl ToString for Attr {
    fn to_string(&self) -> std::string::String {
        format!("{}=\"{}\"", self.name, self.value)
    }
}

pub fn emit_graph_start(attrs: &Vec<Attr>) {
    println!("digraph {{");
    println!(
        "graph [{}]",
        attrs
            .iter()
            .map(Attr::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
}

pub fn emit_graph_end() {
    println!("}}")
}

pub fn emit_cluster_start(cluster_name: &str, attrs: &Vec<Attr>) {
    println!("subgraph cluster_{} {{", cluster_name);
    println!(
        "{}",
        attrs
            .iter()
            .map(Attr::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn emit_cluster_end() {
    println!("}}");
}

pub fn emit_node(name: &str, attrs: &Vec<Attr>) {
    println!(
        "{} [{}]",
        name,
        attrs
            .iter()
            .map(Attr::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
}

pub fn emit_edge(from_node: &String, to_node: &String, attrs: &Vec<Attr>) {
    println!(
        "{} -> {} [{}]",
        from_node,
        to_node,
        attrs
            .iter()
            .map(Attr::to_string)
            .collect::<Vec<String>>()
            .join(" "),
    );
}

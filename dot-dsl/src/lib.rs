pub mod graph {
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }
    impl  Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new()
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().fold(HashMap::new(), |mut acc, &x| {
                acc.insert(x.0.to_string(), x.1.to_string());
                acc
            });
            self
        }
        pub fn get_node(&self, node: &str) -> Option<&Node> {
            self.nodes.iter().find(|&x| x.node == node)
        }
    }
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                node1: String,
                node2: String,
                attrs: HashMap<String, String>
            }
            impl Edge  {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Edge {
                        node1: node1.to_string(),
                        node2: node2.to_string(),
                        attrs: HashMap::new()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().fold(HashMap::new(), |mut acc, &x| {
                        acc.insert(x.0.to_string(), x.1.to_string());
                        acc
                    });
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub(crate) node: String,
                attrs: HashMap<String, String>
            }
            impl Node{
                pub fn new(node: &str) -> Self {
                    Node {
                        node: node.to_string(),
                        attrs: HashMap::new()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().fold(HashMap::new(), |mut acc, &x| {
                        acc.insert(x.0.to_string(), x.1.to_string());
                        acc
                    });
                    self
                }
                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    match self.attrs.get(attr) {
                        Some(key) => Some(key),
                        _ => None
                    }
                }
            }
        }
    }
}

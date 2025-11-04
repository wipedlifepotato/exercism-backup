//use dot_dsl::graph::graph_items::edge::Edge;
//use dot_dsl::graph::graph_items::node::Node;
//use dot_dsl::graph::Graph;

pub mod graph {

    pub mod graph_items {
        pub mod edge {
            #[derive(Clone,Debug,PartialEq)]
            pub struct Edge {
                attrs: std::collections::HashMap<String, String>,
                a: String,
                b: String, // there would be pair
            }
            impl Edge {
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
                pub fn new(a: &str, b: &str) -> Self {
                    
                    Edge { 
                        a:a.to_string(), b:b.to_string(),
                        attrs: std::collections::HashMap::<String, String>::new(),
                    }
                }
            }
        }
        pub mod node {
            #[derive(Clone,Debug,PartialEq)]
            pub struct Node {
                pub val: String,
                pub attrs: std::collections::HashMap<String, String>,
            }
            impl Node {
                pub fn new(val: &str) -> Self {
                    Node { val: val.to_string(),attrs: std::collections::HashMap::new() }
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
                // let attrs: &[(&str, &str)] = &[("color", "blue"), ("label", "start")];
                /*let triples: &[(&str, &str, &str)] = &[
                        ("id", "color", "blue"),
                        ("id", "label", "start"),
                        ("id", "shape", "circle"),
                    ];
                 */
                // Slice type
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    let mut map = std::collections::HashMap::new();
                    for (k, v) in attrs {
                        map.insert(k.to_string(), v.to_string());
                    }
                    self.attrs = map;
                    self
                }
            }
        }
    }
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;
    #[derive(Clone,Debug,PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: std::collections::HashMap<String, String>
    }
    use std::fmt::Formatter;
    impl std::fmt::Display for Graph  {
        fn fmt(&self, _: &mut Formatter::<'_>) -> Result<(), std::fmt::Error> {
            print!("nodes: {:?}; edges: {:?}; attrs: {:?}", &self.nodes, &self.edges, &self.attrs);
            Ok(())
        }       
    }
    impl Graph {

        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: std::collections::HashMap::new(),
            }
        }
        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.val == name)
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }


    }
}

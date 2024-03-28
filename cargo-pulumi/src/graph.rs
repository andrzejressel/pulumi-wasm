// From https://github.com/sfackler/cargo-tree/blob/master/src/graph.rs
use std::collections::HashMap;

use anyhow::Error;
use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;

#[derive(Debug)]
pub struct Graph {
    pub graph: StableGraph<Package, DependencyKind>,
    pub nodes: HashMap<PackageId, NodeIndex>,
    pub root: Option<PackageId>,
}

pub fn build(metadata: Metadata) -> Result<Graph, Error> {
    let resolve = metadata.resolve.unwrap();

    let mut graph = Graph {
        graph: StableGraph::new(),
        nodes: HashMap::new(),
        root: resolve.root,
    };

    for package in metadata.packages {
        let id = package.id.clone();
        let index = graph.graph.add_node(package);
        graph.nodes.insert(id, index);
    }

    for node in resolve.nodes {
        let from = graph.nodes[&node.id];
        for dep in node.deps {
            let to = graph.nodes[&dep.pkg];
            for kind in dep.dep_kinds {
                if kind.kind != DependencyKind::Normal {
                    continue;
                }

                graph.graph.add_edge(from, to, kind.kind);
            }
        }
    }

    Ok(graph)
}

use serde::{Deserialize, Serialize};
use cargo_metadata::{Metadata, PackageId,  Package};

/**
root-workspace_members-dependency
 */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MetadataTreeGraph {
    pub metadata: Metadata,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MetadataGraph {
    /// A list of all crates referenced by this crate (and the crate itself)
    pub nodes: Vec<Package>,
    pub edges: Vec<MetadataGraphNodeEdge>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MetadataGraphNodeEdge {
    pub source: PackageId,
    pub target: PackageId,
    pub value: u16,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MetadataTree {
    
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct G6Metadata {
    pub metadata: Metadata,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum G6Json {
    #[serde(rename = "graph")]
    MetadataGraph { nodes: Vec<Package>, edges: Vec<MetadataGraphNodeEdge> },
    #[serde(rename = "tree")]
    MetadataTree {},
    #[serde(rename = "meta")]
    Metadata { metadata: Metadata },
}


impl MetadataTreeGraph {
    pub fn new(metadata: Metadata) -> Self {
        MetadataTreeGraph { metadata }
    }

    pub fn tree(&self) -> G6Json {
        // TODO: impl tree node
        G6Json::MetadataTree {}
    }

    pub fn meta(&self) -> G6Json {
        G6Json::Metadata { metadata: self.metadata.clone() }
    }


    pub fn graph(&self) -> G6Json {
        let mut metadata_graph = MetadataGraph { nodes: vec![], edges: vec![] };

        metadata_graph.nodes = self.metadata.packages.clone();

        self.metadata.resolve.iter()
            .map(|s| s.nodes.clone())
            .flatten().for_each(|node| {
            let source = &node.id;

            for target in node.dependencies {
                metadata_graph.edges.push(MetadataGraphNodeEdge {
                    source: source.clone(),
                    target,
                    value: 1,
                })
            }
        });
        G6Json::MetadataGraph {
            nodes: metadata_graph.nodes,
            edges: metadata_graph.edges,
        }
    }
}
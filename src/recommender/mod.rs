use std::collections::HashMap;
use std::fmt;

mod graph;
use self::graph::Graph;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum RecommenderNode {
    Tag(String),
    Object(String)
}

pub struct Recommender {
    graph: Graph<RecommenderNode>
}

impl Recommender {
    pub fn new() -> Recommender {
        Recommender { graph: Graph::new() }
    }

    pub fn add_object(&mut self, object: &str) {
        self.graph.add_node(&RecommenderNode::Object(String::from(object)));
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.graph.add_node(&RecommenderNode::Tag(String::from(tag)));
    }

    pub fn tag_object(&mut self, object: &str, tag: &str) {
        self.graph.add_edge(
            &RecommenderNode::Object(String::from(object)),
            &RecommenderNode::Tag(String::from(tag))
        );
    }

    fn simple_recommendations_map(&self, from: &RecommenderNode, iterations: u8, depth: u8) -> HashMap<RecommenderNode, u32> {
        let mut acc: HashMap<RecommenderNode, u32> = HashMap::new();
        for _ in 0..iterations {
            let walk = self.graph.random_walk_simple(from, depth);
            for visited in walk {
                let count = acc.entry(visited).or_insert(0);
                *count += 1;
            }
        }
        acc
    }

    pub fn simple_recommendations(&self, from: &RecommenderNode, iterations: u8, depth: u8) -> Vec<RecommenderNode> {
        let all_recommendations = self.simple_recommendations_map(
            from, iterations, depth);
        let mut top_recommendations = all_recommendations.iter()
            .collect::<Vec<(&RecommenderNode, &u32)>>();
        top_recommendations.sort_by_key(|(_, &v)| v);
        top_recommendations.reverse();
        top_recommendations.iter()
            .cloned()
            .map(|(k, _)| k.clone())
            .collect()
    }
}

impl fmt::Debug for Recommender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Recommender [{:?}]", self.graph)
    }
}

extern crate rand;
mod graph;
use graph::Graph;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum RecommenderNode {
    Tag(String),
    Object(String)
}

struct Recommender {
    graph: Graph<RecommenderNode>
}

impl Recommender {
    fn new() -> Recommender {
        Recommender { graph: Graph::new() }
    }

    fn add_object(&mut self, object: &str) {
        self.graph.add_node(&RecommenderNode::Object(String::from(object)));
    }

    fn add_tag(&mut self, tag: &str) {
        self.graph.add_node(&RecommenderNode::Tag(String::from(tag)));
    }

    fn tag_object(&mut self, object: &str, tag: &str) {
        self.graph.add_edge(
            &RecommenderNode::Object(String::from(object)),
            &RecommenderNode::Tag(String::from(tag))
        );
    }

    fn simple_recommendations(&self, from: &RecommenderNode, iterations: u8) -> HashMap<RecommenderNode, u32> {
        let walk = self.graph.random_walk_simple(from, iterations);
        let mut acc: HashMap<RecommenderNode, u32> = HashMap::new();
        for visited in walk {
            let count = acc.entry(visited).or_insert(0);
            *count += 1;
        }
        acc
    }
}

fn main() {
    let mut recommender: Recommender = Recommender::new();

    recommender.add_object("Star Wars");
    recommender.add_object("007");
    recommender.add_tag("Action");
    recommender.add_tag("Sci-fi");

    recommender.tag_object("Star Wars", "Sci-fi");
    recommender.tag_object("Star Wars", "Action");
    recommender.tag_object("007", "Action");

    let recommendations = recommender.simple_recommendations(
        &RecommenderNode::Object(String::from("Star Wars")),
        20);

    println!("Graph: {:?}", recommender.graph);
    println!("Recommendations: {:?}", recommendations);
}

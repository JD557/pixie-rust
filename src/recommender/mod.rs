use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

mod graph;
use self::graph::Graph;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum RecommenderNode<T> {
    Tag(String),
    Object(T)
}

pub struct Recommender<T> {
    graph: Graph<RecommenderNode<T>>
}

impl<T : Eq + Clone + Hash> Recommender<T> {
    pub fn new() -> Recommender<T> {
        Recommender { graph: Graph::new() }
    }

    pub fn add_object(&mut self, object: &T) {
        self.graph.add_node(&RecommenderNode::Object(object.clone()));
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.graph.add_node(&RecommenderNode::Tag(String::from(tag)));
    }

    pub fn tag_object(&mut self, object: &T, tag: &str) {
        self.graph.add_edge(
            &RecommenderNode::Object(object.clone()),
            &RecommenderNode::Tag(String::from(tag))
        );
    }

    fn recommendations_map(&self,
                           from: &RecommenderNode<T>,
                           iterations: u8,
                           depth: u8,
                           weight_fun: &Fn(&RecommenderNode<T>,&RecommenderNode<T>) -> f32) -> HashMap<RecommenderNode<T>, u32> {
        let mut acc: HashMap<RecommenderNode<T>, u32> = HashMap::new();
        for _ in 0..iterations {
            let walk = self.graph.random_walk(from, depth, weight_fun);
            for visited in walk {
                let count = acc.entry(visited).or_insert(0);
                *count += 1;
            }
        }
        acc
    }

    pub fn recommendations(&self,
                                  from: &RecommenderNode<T>,
                                  iterations: u8,
                                  depth: u8,
                                  weight_fun: &Fn(&RecommenderNode<T>,&RecommenderNode<T>) -> f32) -> Vec<RecommenderNode<T>> {
        let all_recommendations = self.recommendations_map(
            from, iterations, depth, weight_fun);
        let mut top_recommendations = all_recommendations.iter()
            .collect::<Vec<(&RecommenderNode<T>, &u32)>>();
        top_recommendations.sort_by_key(|(_, &v)| v);
        top_recommendations.reverse();
        top_recommendations.iter()
            .cloned()
            .map(|(k, _)| k.clone())
            .collect()
    }
}

impl<T: Eq + Hash + fmt::Debug> fmt::Debug for Recommender<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Recommender [{:?}]", self.graph)
    }
}

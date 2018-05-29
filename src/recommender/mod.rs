use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::vec::Vec;

mod graph;
use self::graph::Graph;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum RecommenderNode<T> {
    Tag(String),
    Object(T),
}

pub struct Recommender<T> {
    graph: Graph<RecommenderNode<T>>,
}

impl<T: Eq + Clone + Hash> Recommender<T> {
    pub fn new() -> Recommender<T> {
        Recommender {
            graph: Graph::new(),
        }
    }

    pub fn add_object(&mut self, object: &T) {
        self.graph
            .add_node(&RecommenderNode::Object(object.clone()));
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.graph
            .add_node(&RecommenderNode::Tag(String::from(tag)));
    }

    pub fn tag_object(&mut self, object: &T, tag: &str) {
        self.graph.add_edge(
            &RecommenderNode::Object(object.clone()),
            &RecommenderNode::Tag(String::from(tag)),
        );
    }

    fn recommendations_map(
        &self,
        from: &RecommenderNode<T>,
        depth: u8,
        max_total_steps: usize,
        weight_fun: &Fn(&RecommenderNode<T>, &RecommenderNode<T>) -> f32,
    ) -> HashMap<RecommenderNode<T>, u32> {
        let mut acc: HashMap<RecommenderNode<T>, u32> = HashMap::new();
        let mut steps_acc = 0;
        while steps_acc < max_total_steps {
            let walk = self.graph.random_walk(from, depth, weight_fun);
            for visited in walk {
                let count = acc.entry(visited).or_insert(0);
                *count += 1;
                steps_acc += 1;
            }
        }
        acc
    }

    pub fn recommendations(
        &self,
        queries: &Vec<&RecommenderNode<T>>,
        depth: u8,
        max_total_steps: usize,
        weight_fun: &Fn(&RecommenderNode<T>, &RecommenderNode<T>) -> f32,
    ) -> Vec<RecommenderNode<T>> {
        let query_scaling_factors = queries
            .iter()
            .map(|q| {
                let degree = self.graph.degree(q) as f64;
                degree * (self.graph.max_degree() as f64 - degree.log2())
            })
            .collect::<Vec<f64>>();

        let total_scaling: f64 = query_scaling_factors.iter().sum();

        let mut all_recommendations: HashMap<RecommenderNode<T>, u32> = HashMap::new();
        for (q, s) in queries.iter().zip(query_scaling_factors.iter()) {
            let max_steps: usize = ((max_total_steps as f64) * s / total_scaling) as usize;
            let query_recommendations = self.recommendations_map(q, depth, max_steps, weight_fun);
            for (key, value) in query_recommendations.iter() {
                all_recommendations
                    .entry(key.clone())
                    .and_modify(|x| *x += value)
                    .or_insert(value.clone());
            }
        }
        let mut top_recommendations = all_recommendations
            .iter()
            .collect::<Vec<(&RecommenderNode<T>, &u32)>>();
        top_recommendations.sort_by_key(|(_, &v)| v);
        top_recommendations.reverse();
        top_recommendations
            .iter()
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

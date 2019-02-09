//! # Recommender
//!
//! The `recommender` module is a collection of utilities to create
//! a recommender and give recommendations.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::vec::Vec;

pub mod graph;
use self::graph::Graph;

/// Nodes to be used for recommendations.
///
/// A node can be either a `Tag` (e.g. a product category) or
/// an `Object` (e.g. a product).
///
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum RecommenderNode<T> {
    Tag(String),
    Object(T),
}

/// A recommender that holds objects, tags and their relationship,
/// and is able to return recommendations.
pub struct Recommender<T> {
    graph: Graph<RecommenderNode<T>>,
}

impl<T: Eq + Clone + Hash> Recommender<T> {
    /// Creates a new recommender.
    pub fn new() -> Recommender<T> {
        Recommender {
            graph: Graph::new(),
        }
    }

    /// Adds an object to this recommender.
    pub fn add_object(&mut self, object: &T) {
        self.graph
            .add_node(&RecommenderNode::Object(object.clone()));
    }

    /// Adds a tag to this recommender.
    pub fn add_tag(&mut self, tag: &str) {
        self.graph
            .add_node(&RecommenderNode::Tag(String::from(tag)));
    }

    /// Assigns a tag to an object.
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
        weight_fun: impl Fn(&RecommenderNode<T>, &RecommenderNode<T>) -> f32,
    ) -> HashMap<RecommenderNode<T>, u32> {
        let mut acc: HashMap<RecommenderNode<T>, u32> = HashMap::new();
        let mut steps_acc = 0;
        while steps_acc < max_total_steps {
            let walk = self.graph.random_walk(from, depth, &weight_fun);
            if walk.len() == 0 {
                return acc;
            }
            for visited in walk {
                let count = acc.entry(visited).or_insert(0);
                *count += 1;
                steps_acc += 1;
            }
        }
        acc
    }

    /// Receives a set of queries (that can be either tags or objects) and
    /// returns an ordered sequence of recommendations (with the first one
    /// being the "best" one).
    ///
    /// The resulting recommendations can be either objects or tags, so it
    /// is advised to filter the result according to the expectations.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixie_rust::recommender::Recommender;
    /// use pixie_rust::recommender::RecommenderNode;
    ///
    /// let mut recommender: Recommender<String> = Recommender::new();
    ///
    /// let raid = String::from("The Raid");
    /// let rocky = String::from("Rocky");
    /// let python = String::from("Monty Python and The Holy Grail");
    ///
    /// let action = String::from("Action");
    /// let comedy = String::from("Comedy");
    /// let drama = String::from("Drama");
    ///
    /// recommender.add_object(&raid);
    /// recommender.add_object(&rocky);
    /// recommender.add_object(&python);
    ///
    /// recommender.add_tag(&action);
    /// recommender.add_tag(&comedy);
    /// recommender.add_tag(&drama);
    ///
    /// recommender.tag_object(&raid, &action);
    /// recommender.tag_object(&rocky, &action);
    /// recommender.tag_object(&rocky, &drama);
    /// recommender.tag_object(&python, &comedy);
    ///
    /// let recommendations = recommender
    ///     .recommendations(
    ///         &vec![RecommenderNode::Tag(action)],
    ///         10,
    ///         10,
    ///         |_, _| 1.0,
    ///         |_, _| 1.0
    ///     )
    ///     .iter()
    ///     .filter(|node| match node {
    ///         RecommenderNode::Tag(_) => false,
    ///         RecommenderNode::Object(_) => true
    ///     })
    ///     .cloned()
    ///     .collect::<Vec<RecommenderNode<String>>>();
    ///
    /// assert!(
    ///     recommendations[0] == RecommenderNode::Object(rocky) ||
    ///     recommendations[0] == RecommenderNode::Object(raid)
    ///     )
    /// ```
    pub fn recommendations(
        &self,
        queries: &Vec<RecommenderNode<T>>,
        depth: u8,
        max_total_steps: usize,
        object_to_tag_weight: impl Fn(&T, &String) -> f32,
        tag_to_object_weight: impl Fn(&String, &T) -> f32,
    ) -> Vec<RecommenderNode<T>> {
        let query_scaling_factors = queries
            .iter()
            .map(|q| {
                let degree = self.graph.degree(q) as f64;
                degree * (self.graph.max_degree() as f64 - degree.log2())
            })
            .collect::<Vec<f64>>();

        let total_scaling: f64 = query_scaling_factors.iter().sum();

        let mut all_recommendations: HashMap<RecommenderNode<T>, f64> = HashMap::new();
        for (q, s) in queries.iter().zip(query_scaling_factors.iter()) {
            let max_steps: usize = ((max_total_steps as f64) * s / total_scaling) as usize;
            let query_recommendations =
                self.recommendations_map(q, depth, max_steps, |from, to| match (from, to) {
                    (RecommenderNode::Tag(tag), RecommenderNode::Object(obj)) => {
                        tag_to_object_weight(tag, obj)
                    }
                    (RecommenderNode::Object(obj), RecommenderNode::Tag(tag)) => {
                        object_to_tag_weight(obj, tag)
                    }
                    _ => 0.0,
                });
            for (key, value) in query_recommendations.iter() {
                let value_sqrt = (value.clone() as f64).sqrt();
                all_recommendations
                    .entry(key.clone())
                    .and_modify(|x| *x += value_sqrt)
                    .or_insert(value_sqrt);
            }
        }

        let mut queries_set: HashSet<&RecommenderNode<T>> = HashSet::new();
        for q in queries {
            queries_set.insert(q);
        }

        let mut top_recommendations = all_recommendations
            .iter()
            .filter(|(k, _)| !queries_set.contains(*k))
            .map(|(k, v)| (k, ((v * v) as u32)))
            .collect::<Vec<(&RecommenderNode<T>, u32)>>();
        top_recommendations.sort_by_key(|(_, v)| *v);
        top_recommendations.reverse();
        top_recommendations
            .iter()
            .map(|(k, _)| k)
            .cloned()
            .cloned()
            .collect()
    }

    /// Receives a set of queries (that can only objects) and
    /// returns an ordered sequence of recommendations (with the first one
    /// being the "best" one).
    ///
    /// This is a simplified version of the `recommendations` operation
    /// that only returns objects.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixie_rust::recommender::Recommender;
    ///
    /// let mut recommender: Recommender<String> = Recommender::new();
    ///
    /// let raid = String::from("The Raid");
    /// let rocky = String::from("Rocky");
    /// let python = String::from("Monty Python and The Holy Grail");
    ///
    /// let action = String::from("Action");
    /// let comedy = String::from("Comedy");
    /// let drama = String::from("Drama");
    ///
    /// recommender.add_object(&raid);
    /// recommender.add_object(&rocky);
    /// recommender.add_object(&python);
    ///
    /// recommender.add_tag(&action);
    /// recommender.add_tag(&comedy);
    /// recommender.add_tag(&drama);
    ///
    /// recommender.tag_object(&raid, &action);
    /// recommender.tag_object(&rocky, &action);
    /// recommender.tag_object(&rocky, &drama);
    /// recommender.tag_object(&python, &comedy);
    ///
    /// let recommendations = recommender
    ///     .object_recommendations(
    ///         &vec![raid.clone()],
    ///         50,
    ///         50,
    ///         |_, _| 1.0,
    ///         |_, _| 1.0
    ///     )
    ///     .iter()
    ///     .cloned()
    ///     .collect::<Vec<String>>();
    ///
    /// assert!(recommendations.len() > 0);
    /// assert!(recommendations[0] == rocky)
    /// ```
    pub fn object_recommendations(
        &self,
        queries: &Vec<T>,
        depth: u8,
        max_total_steps: usize,
        object_to_tag_weight: impl Fn(&T, &String) -> f32,
        tag_to_object_weight: impl Fn(&String, &T) -> f32,
    ) -> Vec<T> {
        let node_queries: Vec<RecommenderNode<T>> = queries
            .iter()
            .map(|x| RecommenderNode::Object(x.clone()))
            .collect();
        self.recommendations(
            &node_queries,
            depth,
            max_total_steps,
            object_to_tag_weight,
            tag_to_object_weight,
        )
        .iter()
        .flat_map(|node| match node {
            RecommenderNode::Tag(_) => None,
            RecommenderNode::Object(obj) => Some(obj.clone()),
        })
        .collect()
    }
}

impl<T: Eq + Hash + fmt::Debug> fmt::Debug for Recommender<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Recommender [{:?}]", self.graph)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dead_end_recommendations_map() {
        let mut recommender: Recommender<String> = Recommender::new();

        let obj_0 = String::from("0.0");
        let tag_1 = String::from("1.0");
        let obj_2 = String::from("2.0");

        recommender.add_object(&obj_0);
        recommender.add_tag(&tag_1);
        recommender.add_object(&obj_2);

        recommender.tag_object(&obj_0, &tag_1);
        recommender.tag_object(&obj_2, &tag_1);

        let recommendations = recommender.recommendations_map(
            &RecommenderNode::Object(obj_0.clone()),
            3,
            3,
            |from, to| match (from, to) {
                (RecommenderNode::Tag(tag), RecommenderNode::Object(obj)) => {
                    obj.parse::<f32>().unwrap() - tag.parse::<f32>().unwrap()
                }
                (RecommenderNode::Object(obj), RecommenderNode::Tag(tag)) => {
                    tag.parse::<f32>().unwrap() - obj.parse::<f32>().unwrap()
                }
                _ => 0.0,
            },
        );

        assert_eq!(
            recommendations.get(&RecommenderNode::Tag(tag_1)).unwrap(),
            &1
        );
        assert_eq!(
            recommendations
                .get(&RecommenderNode::Object(obj_2))
                .unwrap(),
            &1
        );
    }
    #[test]
    fn basic_recommendations() {
        let mut recommender: Recommender<String> = Recommender::new();

        let obj_0 = String::from("0.0");
        let tag_1 = String::from("1.0");
        let obj_2 = String::from("2.0");

        recommender.add_object(&obj_0);
        recommender.add_tag(&tag_1);
        recommender.add_object(&obj_2);

        recommender.tag_object(&obj_0, &tag_1);
        recommender.tag_object(&obj_2, &tag_1);

        let recommendations = recommender
            .recommendations(
                &vec![RecommenderNode::Object(obj_0.clone())],
                10,
                10,
                |from, to| to.parse::<f32>().unwrap() - from.parse::<f32>().unwrap(),
                |from, to| to.parse::<f32>().unwrap() - from.parse::<f32>().unwrap(),
            )
            .iter()
            .cloned()
            .collect::<HashSet<RecommenderNode<String>>>();

        assert!(!recommendations.contains(&RecommenderNode::Object(obj_0)));
        assert!(recommendations.contains(&RecommenderNode::Tag(tag_1)));
        assert!(recommendations.contains(&RecommenderNode::Object(obj_2)));
    }
}

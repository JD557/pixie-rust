//! # Graph
//!
//! The `graph` module is a collection of utilities to handle an
//! undirected graph structure.
//!
//! The end user of the library should not need to use this module
//! directly.

extern crate rand;
use rand::rngs::OsRng;
use rand::Rng;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;

/// Data structure containing an undirected graph.
pub struct Graph<T> {
    data: HashMap<T, HashSet<T>>,
    max_degree: usize,
}

impl<T: Eq + Clone + Hash> Graph<T> {
    /// Creates an empty graph
    pub fn new() -> Graph<T> {
        Graph {
            data: HashMap::new(),
            max_degree: 0,
        }
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: &T) {
        self.data.entry(node.clone()).or_insert(HashSet::new());
    }

    /// Adds an edge to the graph. The nodes are created, if needed.
    pub fn add_edge(&mut self, node_a: &T, node_b: &T) {
        let degree_a = self
            .data
            .entry(node_a.clone())
            .and_modify(|e| {
                e.insert(node_b.clone());
            }).or_insert({
                let mut h = HashSet::new();
                h.insert(node_b.clone());
                h
            }).len();
        let degree_b = self
            .data
            .entry(node_b.clone())
            .and_modify(|e| {
                e.insert(node_a.clone());
            }).or_insert({
                let mut h = HashSet::new();
                h.insert(node_a.clone());
                h
            }).len();

        if degree_a > self.max_degree {
            self.max_degree = degree_a;
        }

        if degree_b > self.max_degree {
            self.max_degree = degree_b;
        }
    }

    /// Lists the successors of a node.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixie_rust::recommender::graph::Graph;
    /// use std::collections::HashSet;
    ///
    /// let mut graph: Graph<u32> = Graph::new();
    ///
    /// graph.add_node(&1);
    /// graph.add_node(&2);
    /// graph.add_edge(&1, &2);
    ///
    /// let mut expected_result: HashSet<u32> = HashSet::new();
    /// expected_result.insert(2);
    /// assert_eq!(graph.successors(&1), expected_result);
    ///
    /// let mut expected_result: HashSet<u32> = HashSet::new();
    /// expected_result.insert(1);
    /// assert_eq!(graph.successors(&2), expected_result);
    /// ```
    pub fn successors(&self, node: &T) -> HashSet<T> {
        self.data.get(node).unwrap_or(&HashSet::new()).clone()
    }

    /// Returns the degree of the node with the largest degree in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixie_rust::recommender::graph::Graph;
    ///
    /// let mut graph: Graph<u32> = Graph::new();
    ///
    /// graph.add_node(&1);
    /// graph.add_node(&2);
    /// graph.add_node(&3);
    /// assert_eq!(graph.max_degree(), 0);
    /// graph.add_edge(&1, &2);
    /// assert_eq!(graph.max_degree(), 1);
    /// graph.add_edge(&1, &3);
    /// assert_eq!(graph.max_degree(), 2);
    /// graph.add_edge(&2, &3);
    /// assert_eq!(graph.max_degree(), 2);
    /// ```
    pub fn max_degree(&self) -> usize {
        self.max_degree
    }

    /// Returns the degree of a node.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixie_rust::recommender::graph::Graph;
    ///
    /// let mut graph: Graph<u32> = Graph::new();
    ///
    /// graph.add_node(&1);
    /// graph.add_node(&2);
    /// graph.add_node(&3);
    /// assert_eq!(graph.degree(&1), 0);
    /// graph.add_edge(&1, &2);
    /// assert_eq!(graph.degree(&1), 1);
    /// graph.add_edge(&1, &3);
    /// assert_eq!(graph.degree(&1), 2);
    /// assert_eq!(graph.degree(&2), 1);
    /// assert_eq!(graph.degree(&3), 1);
    /// ```
    pub fn degree(&self, node: &T) -> usize {
        self.data.get(node).map(|x| x.len()).unwrap_or(0)
    }

    fn weighted_sample(
        rng: &mut impl Rng,
        elems: LinkedList<&T>,
        weight_fun: &Fn(&T) -> f32,
    ) -> Option<T> {
        let safe_weight_fun: &Fn(&T) -> f32 = &(|x| {
            let unsafe_weight = weight_fun(x);
            let clamped_weight = unsafe_weight.max(0.0);
            if clamped_weight.is_infinite() {
                0.0
            } else {
                clamped_weight
            }
        });
        let total_weight: f32 = elems.iter().map(|e| safe_weight_fun(e)).sum();
        if total_weight == 0.0 {
            None
        } else {
            let mut goal: f32 = rng.gen_range(0.0, total_weight);
            let mut iterator = elems.iter();
            let mut choice: Option<&&T> = iterator.next();
            while choice.is_some() {
                let value = choice.unwrap();
                goal = goal - safe_weight_fun(value);
                if goal <= 0.0 {
                    break;
                } else {
                    choice = iterator.next();
                }
            }
            choice.cloned().cloned()
        }
    }

    /// Performs a random walk on a graph.
    /// It picks the next node according to a weight function
    /// `(from, to) = weight`.
    ///
    /// It returns the list of visited nodes in reverse order.
    ///
    /// # Examples
    ///
    /// ```
    /// use pixie_rust::recommender::graph::Graph;
    ///
    /// let mut graph: Graph<u32> = Graph::new();
    ///
    /// graph.add_node(&1);
    /// graph.add_node(&2);
    /// graph.add_node(&3);
    /// graph.add_edge(&1, &2);
    /// graph.add_edge(&1, &3);
    /// let visited = graph.random_walk(&1, 200, &(|_, x| x.clone() as f32));
    /// assert_eq!(visited.len(), 200);
    ///
    /// // The node 3 should be visited more often due to the weight function
    /// assert!(
    ///     visited.iter().filter(|&&x| x == 2).count() <
    ///     visited.iter().filter(|&&x| x == 3).count()
    /// );
    /// ```
    pub fn random_walk(
        &self,
        starting_node: &T,
        max_hops: u8,
        weight_fun: &Fn(&T, &T) -> f32,
    ) -> LinkedList<T> {
        let mut rng = OsRng::new().expect("Failed to create the RNG");
        let mut visited: LinkedList<T> = LinkedList::new();
        if self.data.contains_key(starting_node) {
            let mut current_node = starting_node.clone();
            let mut hops = max_hops;
            while hops > 0 {
                hops = hops - 1;
                visited.push_front(current_node.clone());
                let succs = self.successors(&current_node);
                let next = Graph::weighted_sample(
                    &mut rng,
                    LinkedList::from_iter(succs.iter()),
                    &(|next_node| weight_fun(&current_node, next_node)),
                );
                match next {
                    None => break,
                    Some(v) => current_node = v.clone(),
                };
            }
        }
        visited
    }
}

impl<T: fmt::Debug + Eq + Hash> fmt::Debug for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Graph {:?} / Max Degree = {:?}",
            self.data, self.max_degree
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unknown_node_random_walk() {
        let graph: Graph<u32> = Graph::new();
        let visited = graph.random_walk(&1, 200, &(|_, x| x.clone() as f32));
        assert_eq!(
            visited.len(),
            0,
            "Visited {} node(s) on an empty graph",
            visited.len()
        );
    }

    #[test]
    fn lone_node_random_walk() {
        let mut graph: Graph<u32> = Graph::new();
        graph.add_node(&1);
        let visited = graph.random_walk(&1, 200, &(|_, x| x.clone() as f32));
        assert_eq!(
            visited.len(),
            1,
            "Visited {} nodes on a graph with a single node",
            visited.len()
        );
    }

    #[test]
    fn sample_with_weights() {
        let mut rng = rand::thread_rng();
        let mut list: LinkedList<&u8> = LinkedList::new();
        list.push_front(&0);
        list.push_front(&1);
        let res1 = Graph::weighted_sample(&mut rng, list.clone(), &(|x| x.clone() as f32));
        assert_eq!(res1.unwrap(), 1);
        let res2 = Graph::weighted_sample(
            &mut rng,
            list.clone(),
            &(|x| 1.0 + (-1.0 * (x.clone() as f32))),
        );
        assert_eq!(res2.unwrap(), 0);
        let res3 = Graph::weighted_sample(&mut rng, list.clone(), &(|_| -1.0));
        assert_eq!(res3, None);
        let res4 = Graph::weighted_sample(&mut rng, list.clone(), &(|_| 1.0));
        assert!(res4.unwrap() == 0 || res4.unwrap() == 1);
    }
}

extern crate rand;
use rand::Rng;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;

pub struct Graph<T> {
    data: HashMap<T, HashSet<T>>,
}

impl<T: Eq + Clone + Hash> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            data: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: &T) {
        self.data.entry(node.clone()).or_insert(HashSet::new());
    }

    pub fn add_edge(&mut self, node_a: &T, node_b: &T) {
        self.data
            .entry(node_a.clone())
            .and_modify(|e| {
                e.insert(node_b.clone());
            })
            .or_insert({
                let mut h = HashSet::new();
                h.insert(node_b.clone());
                h
            });
        self.data
            .entry(node_b.clone())
            .and_modify(|e| {
                e.insert(node_a.clone());
            })
            .or_insert({
                let mut h = HashSet::new();
                h.insert(node_a.clone());
                h
            });
    }

    pub fn successors(&self, node: &T) -> HashSet<T> {
        self.data.get(node).unwrap_or(&HashSet::new()).clone()
    }

    fn weighted_sample(elems: LinkedList<T>, weight_fun: &Fn(&T) -> f32) -> Option<T> {
        let total_weight: f32 = elems.iter().map(|e| weight_fun(e)).sum();
        let mut rng = rand::thread_rng();
        let mut goal: f32 = rng.gen_range(0.0, total_weight);
        let mut iterator = elems.iter().cloned();
        let mut choice = iterator.next();
        while choice.is_some() {
            let value = choice.clone().unwrap();
            goal = goal - weight_fun(&value);
            if goal <= 0.0 {
                break;
            } else {
                choice = iterator.next();
            }
        }
        choice
    }

    pub fn random_walk(
        &self,
        starting_node: &T,
        max_hops: u8,
        weight_fun: &Fn(&T, &T) -> f32,
    ) -> LinkedList<T> {
        let mut visited: LinkedList<T> = LinkedList::new();
        let mut current_node = starting_node.clone();
        let mut hops = max_hops;
        while hops > 0 {
            hops = hops - 1;
            visited.push_front(current_node.clone());
            let succs = self.successors(&current_node);
            let next = Graph::weighted_sample(
                LinkedList::from_iter(succs.iter().cloned()),
                &(|next_node| weight_fun(&current_node, next_node)),
            );
            match next {
                None => break,
                Some(v) => current_node = v.clone(),
            };
        }
        visited
    }
}

impl<T: fmt::Debug + Eq + Hash> fmt::Debug for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph {:?}", self.data)
    }
}

extern crate rand;
use rand::Rng;

use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fmt;

struct Graph {
    data: HashMap<String, HashSet<String>>
}

impl Graph {
    fn new() -> Graph {
        Graph { data: HashMap::new() }
    }

    fn add_node(&mut self, node: &str) {
        self.data.entry(String::from(node)).or_insert(HashSet::new());
    }

    fn add_edge(&mut self, node_a: &str, node_b: &str) {
        self.data.entry(String::from(node_a))
            .and_modify(|e| {e.insert(String::from(node_b));})
            .or_insert({let mut h = HashSet::new(); h.insert(String::from(node_b)); h});
        self.data.entry(String::from(node_b))
            .and_modify(|e| {e.insert(String::from(node_a));})
            .or_insert({let mut h = HashSet::new(); h.insert(String::from(node_a)); h});
    }

    fn successors(&self, node: &str) -> HashSet<String> {
        self.data.get(&String::from(node)).unwrap_or(&HashSet::new()).clone()
    }

    fn weighted_sample<T : Clone>(elems: LinkedList<T>, weight_fun: &Fn(&T) -> f32) -> Option<T> {
        let total_weight: f32 = elems.iter().map(|e| weight_fun(e)).sum();
        let mut rng = rand::thread_rng();
        let mut goal: f32 = rng.gen_range(0.0, total_weight);
        let mut iterator = elems.iter().cloned();
        let mut choice = iterator.next();
        while choice.is_some() {
            let value = choice.clone().unwrap();
            goal = goal - weight_fun(&value);
            if goal <= 0.0 {break;}
            else {choice = iterator.next();}
        }
        choice
    }

    fn random_walk(&self, starting_node: &str, max_hops: u8, weight_fun: &Fn(&String) -> f32) -> LinkedList<String> {
        let mut visited: LinkedList<String> = LinkedList::new();
        let mut current_node = String::from(starting_node);
        let mut hops = max_hops;
        while hops > 0 {
            hops = hops - 1;
            visited.push_front(current_node.clone());
            let succs = self.successors(&current_node);
            let next = Graph::weighted_sample::<String>(LinkedList::from_iter(succs.iter().cloned()), weight_fun);
            match next {
                None => break,
                Some(v) => current_node = v.clone()
            };
        }
        visited
    }

    fn random_walk_simple(&self, starting_node: &str, max_hops: u8) -> LinkedList<String> {
        self.random_walk(starting_node, max_hops, &(|_| 1.0))
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph {:?}", self.data)
    }
}

fn main() {
    let mut graph: Graph = Graph::new();
    graph.add_node("a");
    graph.add_node("b");
    graph.add_node("c");
    graph.add_node("d");
    graph.add_node("e");
    graph.add_edge("a", "b");
    graph.add_edge("b", "c");
    graph.add_edge("c", "d");
    graph.add_edge("d", "e");
    graph.add_edge("e", "a");
    let walk = graph.random_walk_simple("a", 20);
    println!("Graph: {:?}", graph);
    println!("Walk: {:?}", walk);
}

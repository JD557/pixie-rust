extern crate rand;
use rand::Rng;

use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

type Graph = HashMap<String, HashSet<String>>;

fn add_node(graph:&mut Graph, node: &str) {
    graph.entry(String::from(node)).or_insert(HashSet::new());
}

fn add_edge(graph:&mut Graph, node_a: &str, node_b: &str) {
    graph.entry(String::from(node_a))
        .and_modify(|e| {e.insert(String::from(node_b));})
        .or_insert({let mut h = HashSet::new(); h.insert(String::from(node_b)); h});
    graph.entry(String::from(node_b))
        .and_modify(|e| {e.insert(String::from(node_a));})
        .or_insert({let mut h = HashSet::new(); h.insert(String::from(node_a)); h});
}

fn successors(graph: &Graph, node: &str) -> HashSet<String> {
    graph.get(&String::from(node)).unwrap_or(&HashSet::new()).clone()
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

fn random_walk(graph: &Graph, starting_node: &str, max_hops: u8, weight_fun: &Fn(&String) -> f32) -> LinkedList<String> {
    let mut visited: LinkedList<String> = LinkedList::new();
    let mut current_node = String::from(starting_node);
    let mut hops = max_hops;
    while hops > 0 {
        hops = hops - 1;
        visited.push_front(current_node.clone());
        let succs = successors(&graph, &current_node);
        let next = weighted_sample::<String>(LinkedList::from_iter(succs.iter().cloned()), weight_fun);
        match next {
            None => break,
            Some(v) => current_node = v.clone()
        };
    }
    visited
}

fn random_walk_simple(graph: &Graph, starting_node: &str, max_hops: u8) -> LinkedList<String> {
    random_walk(graph, starting_node, max_hops, &(|_| 1.0))
}

fn main() {
    let mut graph: Graph = HashMap::new();
    add_node(&mut graph, "a");
    add_node(&mut graph, "b");
    add_node(&mut graph, "c");
    add_node(&mut graph, "d");
    add_node(&mut graph, "e");
    add_edge(&mut graph, "a", "b");
    add_edge(&mut graph, "b", "c");
    add_edge(&mut graph, "c", "d");
    add_edge(&mut graph, "d", "e");
    add_edge(&mut graph, "e", "a");
    let walk = random_walk_simple(&graph, "a", 20);
    println!("Graph: {:?}", graph);
    println!("Walk: {:?}", walk);
}

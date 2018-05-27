extern crate rand;
mod graph;
use graph::Graph;

fn main() {
    let mut graph: Graph<String> = Graph::new();
    graph.add_node(&String::from("a"));
    graph.add_node(&String::from("b"));
    graph.add_node(&String::from("c"));
    graph.add_node(&String::from("d"));
    graph.add_node(&String::from("e"));
    graph.add_edge(&String::from("a"), &String::from("b"));
    graph.add_edge(&String::from("b"), &String::from("c"));
    graph.add_edge(&String::from("c"), &String::from("d"));
    graph.add_edge(&String::from("d"), &String::from("e"));
    graph.add_edge(&String::from("e"), &String::from("a"));
    let walk = graph.random_walk_simple(&String::from("a"), 20);
    println!("Graph: {:?}", graph);
    println!("Walk: {:?}", walk);
}

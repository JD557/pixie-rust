extern crate rand;
mod graph;
use graph::Graph;

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

extern crate rand;

mod recommender;
use recommender::Recommender;
use recommender::RecommenderNode;

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

    println!("Recommender: {:?}", recommender);
    println!("Recommendations: {:?}", recommendations);
}

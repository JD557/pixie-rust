extern crate rand;

use std::fs::File;
use std::io::BufReader;

mod recommender;
use recommender::Recommender;
use recommender::RecommenderNode;

extern crate csv;

fn main() {
    let mut recommender: Recommender = Recommender::new();

    let file = File::open("anime.csv").unwrap();
    let buf_reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(buf_reader);

    for entry_res in csv_reader.records() {
        let entry = entry_res.unwrap();
        let name = entry.get(1).unwrap();
        let categories_str = entry.get(2).unwrap();
        recommender.add_object(name);
        let categories = categories_str.split(",");
        for cat in categories {
            let trimmed = cat.trim();
            recommender.add_tag(trimmed);
            recommender.tag_object(name, trimmed);
        }
    }

    let recommendations = recommender.simple_recommendations(
        &RecommenderNode::Object(String::from("Cowboy Bebop")),
        20);

    //println!("Recommender: {:?}", recommender);
    println!("Recommendations: {:?}", recommendations);
}

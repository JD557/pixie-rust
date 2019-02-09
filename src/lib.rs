//! A Recommender based on [Pinterest's Pixie Recommender][pixie].
//!
//!
//! # Basic usage
//!
//! The quickest way to generate recommendations is by creating a [`Recommender`],
//! adding objects and tags and calling the by using the [`object_recommendations`]
//! method.
//!
//! ```
//! use pixie_rust::recommender::Recommender;
//!
//! let mut recommender: Recommender<String> = Recommender::new();
//!
//! let raid = String::from("The Raid");
//! let rocky = String::from("Rocky");
//! let python = String::from("Monty Python and The Holy Grail");
//!
//! let action = String::from("Action");
//! let comedy = String::from("Comedy");
//! let drama = String::from("Drama");
//!
//! // Add movies
//! recommender.add_object(&raid);
//! recommender.add_object(&rocky);
//! recommender.add_object(&python);
//!
//! // Add genres
//! recommender.add_tag(&action);
//! recommender.add_tag(&comedy);
//! recommender.add_tag(&drama);
//!
//! // Assign genres to movies
//! recommender.tag_object(&raid, &action);
//! recommender.tag_object(&rocky, &action);
//! recommender.tag_object(&rocky, &drama);
//! recommender.tag_object(&python, &comedy);
//!
//! // Ask for movies simmilar to "The Raid"
//! let recommendations = recommender
//!     .object_recommendations(
//!         &vec![raid.clone()],
//!         10,
//!         10,
//!         |_, _| 1.0,
//!         |_, _| 1.0
//!     )
//!     .iter()
//!     .cloned()
//!     .collect::<Vec<String>>();
//! ```
//!
//! [pixie]: https://dl.acm.org/citation.cfm?id=3186183
//! [`Recommender`]: recommender/struct.Recommender.html
//! [`object_recommendations`]: recommender/struct.Recommender.html#method.object_recommendations
//!

extern crate rand;

pub mod recommender;

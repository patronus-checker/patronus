extern crate patronus;

use patronus::{Patronus, Properties};

fn main() {
    let sentence = "Tou manny misteaks woudl confuez an horse. Naturally, mistakes are good.";
    let lang = "en";

    let checker = Patronus::new();
    let properties = Properties {
        primary_language: String::from(lang),
    };

    for provider in &checker.providers {
        println!("{}", provider.name());
    }
    println!("checking {}", sentence);
    println!("{:?}", checker.check(&properties, &sentence.into()));
}

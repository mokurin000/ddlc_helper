use once_cell::sync::Lazy;
use std::collections::HashSet;

enum Charactor {
    Sayori,
    Yuri,
    Natsuki,
}

mod data;

static sayori_words_set: Lazy<HashSet<&'static str>> = Lazy::new(||{
    use data::sayori_words;

    sayori_words.iter()
                .map(|refe| *refe)
                .collect()
});



fn filter_words<'a>(words: &[&'a str]) -> Vec<&'a str> {
    vec!["lol"]
}

fn main() {
    let mut charactor;

    println!("Please select the charactor you like: (enter the number near names)");
    println!();
    println!("0 Sayori, 1 Yuri, 2 Natsuki");
    println!();

    charactor = {
        let num = 
    }

}
#![feature(once_cell)]

mod data;
use data::*;

use std::io::stdin;

#[derive(Copy, Clone)]
enum Charactor {
    Sayori,
    Yuri,
    Natsuki,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Please select the charactor you like: (enter the number near names)");
    println!("0 Sayori, 1 Yuri, 2 Natsuki\n");

    let charactor = {
        let mut buf = String::new();

        stdin().read_line(&mut buf)?;

        let num = buf.trim().parse::<u32>()?;

        match num {
            0 => Charactor::Sayori,
            1 => Charactor::Yuri,
            2 => Charactor::Natsuki,
            _ => return Err("charactor code was out of bound!".into()),
        }
    };

    println!("Great! Now please input the words in a line.");

    let mut raw_words = String::new();
    loop {
        stdin().read_line(&mut raw_words)?;

        let words_list = raw_words.split_whitespace();
        let result = filter_words(words_list, charactor);

        if !result.is_empty() {
            print!("\nResult: ");
            for word in result {
                print!("{} ", word);
            }
            println!("\n");
        }
        
        raw_words.clear();
    }
}

fn filter_words<'a>(
    words: impl Iterator<Item = &'a str>,
    charactor: Charactor,
) -> Vec<&'static str> {
    let words_set = match charactor {
        Charactor::Sayori => &SAYORI_WORDS_SET,
        Charactor::Yuri => &YURI_WORDS_SET,
        Charactor::Natsuki => &NATSUKI_WORDS_SET,
    };

    words
        .map(|s| s.to_lowercase())
        .filter_map(|s| -> Option<&str> {
            if let Some(&word) = words_set.get(&*s) {
                Some(word)
            } else {
                None
            }
        })
        .collect()
}

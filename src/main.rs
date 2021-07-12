enum Charactor {
    Sayori,
    Yuri,
    Natsuki,
}

mod data;
use data::SAYORI_WORDS_SET;
use data::YURI_WORDS_SET;
use data::NATSUKI_WORDS_SET;

use std::io::Read;
use std::io::stdin;

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
            _ => panic!("charactor code was out of bound!"),
        }
    };

    println!("Great! Now please input your words until EOF.");

    let mut raw_words = String::new();
    stdin().read_to_string(&mut raw_words)?;

    let words_list = raw_words.split_whitespace();
    let result = filter_words(words_list, charactor);

    println!("Result:");
    for (index, word) in result.into_iter().enumerate() {
        if index % 2 == 0 {
            println!("{}", word);
        } else {
            print!("{}",word);
        }
    }
    println!();

    Ok(())
}

fn filter_words<'a> (
    words: impl Iterator<Item=&'a str>,
    charactor: Charactor)
-> Vec<String> {

    let words_set = match charactor {
        Charactor::Sayori => &SAYORI_WORDS_SET,
        Charactor::Yuri => &YURI_WORDS_SET,
        Charactor::Natsuki => &NATSUKI_WORDS_SET,
    };

    words
    .map(|s| s.to_lowercase() )
    .filter(|s| words_set.contains::<str>(&s))
    .collect()
}
use ::ddlc_helper::NATSUKI_WORDS_SET;
use ::ddlc_helper::SAYORI_WORDS_SET;
use ::ddlc_helper::YURI_WORDS_SET;

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

    let mut buf = String::new();

    let charactor = {
        stdin().read_line(&mut buf)?;

        let num = buf.trim().parse::<u32>()?;

        match num {
            0 => Charactor::Sayori,
            1 => Charactor::Yuri,
            2 => Charactor::Natsuki,
            _ => return Err("charactor code was out of bound!".into()),
        }
    };

    buf.clear();

    println!("Great! Now please input the words in a line.");

    loop {
        loop {
            stdin().read_line(&mut buf)?;

            if buf.lines().last() == Some("") {
                break;
            }
        }

        let words_list = buf.split_whitespace();
        let result = filter_words(words_list, charactor);

        print!("\nResult: ");
        for word in result {
            print!("{} ", word);
        }
        println!("\n");

        buf.clear();
    }
}

fn filter_words<'a>(
    words: impl Iterator<Item = &'a str>,
    charactor: Charactor,
) -> impl Iterator<Item = &'a str> {
    let words_set = match charactor {
        Charactor::Sayori => &SAYORI_WORDS_SET,
        Charactor::Yuri => &YURI_WORDS_SET,
        Charactor::Natsuki => &NATSUKI_WORDS_SET,
    };

    words.filter(move |word| words_set.contains(&word.to_lowercase()))
}

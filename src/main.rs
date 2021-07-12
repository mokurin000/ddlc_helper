#[cfg(test)]
mod test{
    #[test]
    fn test_sayori_words() {

    }
}

use once_cell::sync::Lazy;
use std::collections::HashSet;

enum Charactor {
    Sayori,
    Yuri,
    Natsuki,
}

static sayori_words_set: Lazy<HashSet<&'static str>> = Lazy::new(||{
    let sayori_words: [&'static str; 88] =[
        "adventure", "alone", "amazing", "awesome",
        "beauty", "bed", "bliss", "broken",
        "calm", "charm", "cheer", "childhood",
        "clumsy", "color", "comfort", "cry",
        "dance", "dark", "daydream", "dazzle",
        "death", "defeat", "depression", "embrace",
        "empty", "excitement", "extraordinary", "family",
        "fear", "feather", "fireflies", "fireworks",
        "flower", "flying", "forgive", "friends",
        "fun", "grief", "happiness", "heart",
        "holiday", "hope", "hopeless", "hurt",
        "joy", "laugh", "lazy", "loud",
        "love", "lucky", "marriage", "memories",
        "misery", "misfortune", "music", "nature",
        "ocean", "pain", "party", "passion",
        "peaceful", "play", "prayer", "precious",
        "promise", "rainbow", "raincloud", "romance",
        "rose", "sadness", "scars", "shame",
        "silly", "sing", "smile", "sparkle",
        "special", "sunny", "sunset", "sweet",
        "tears", "together", "tragedy", "treasure",
        "unrequited", "vacation", "warm", "wonderful"
    ];

    sayori_words.iter()
                .map(|refe| *refe)
                .collect()
});



fn main() {
    
}

fn count_valid_words(sentence: String) -> i32 {
    let rule1 = |word: &str| {
        word.rfind(char::is_numeric) == None
    };

    let rule2 = |word: &str| {
        if let Some(index) = word.find("-") {
            index > 0 && index < word.len() - 1 &&
            word.matches("-").count() == 1 &&
            word.chars().nth(index + 1).unwrap().is_ascii_lowercase()
        } else {
            true
        }
    };

    let rule3 = |word: &str| {
        let punctuations: &[_] = &['!', '.', ','];
        if let Some(index) = word.find(punctuations) {
            index == word.len() - 1
        } else {
            true
        }
    };

    sentence.split_whitespace().filter(|word| 
        rule1(word) && rule2(word) && rule3(word)
    ).count() as i32
}

fn main() {
    let sentence = String::from("cat1 and -invalid dog!");
    assert_eq!(count_valid_words(sentence), 2);

    let sentence = String::from("!this  1-s b8d!");
    assert_eq!(count_valid_words(sentence), 0);

    let sentence = String::from("alice and  bob are playing stone-game10");
    assert_eq!(count_valid_words(sentence), 5);

    let sentence = String::from("he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.");
    assert_eq!(count_valid_words(sentence), 6);

    let sentence = String::from("!");
    assert_eq!(count_valid_words(sentence), 1);

    let sentence = String::from("a--b");
    assert_eq!(count_valid_words(sentence), 0);

    let sentence = String::from("q-,");
    assert_eq!(count_valid_words(sentence), 0);
}

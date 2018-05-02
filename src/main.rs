use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word_list_to_check_filename = &args[1];
    let mut compound_safe_list_output = format!("{}.compound-safe", &word_list_to_check_filename);
    if args.len() == 3 {
        compound_safe_list_output = args[2].to_string();
    }
    let (single_bad_words, double_bad_words, must_remove_words) = split_and_search(
        make_vec(word_list_to_check_filename),
        word_list_to_check_filename,
    );
    let words_to_remove =
        find_words_to_remove(single_bad_words, double_bad_words, must_remove_words);

    println!("Making compound-safe list");
    let clean_word_list = make_clean_list(words_to_remove, make_vec(word_list_to_check_filename));

    let mut f = File::create(&compound_safe_list_output).expect("Unable to create file");
    for i in &clean_word_list {
        writeln!(f, "{}", i).expect("Unable to write data to file");
    }

    println!("");
    println!("------------------------");
    println!("");
    let original_list_length = make_vec(word_list_to_check_filename).len() as f64;
    let clean_list_length = clean_word_list.len() as f64;
    println!(
        "You're inputted word list had {} words ({} bits per word).",
        original_list_length,
        log_base(2, original_list_length)
    );
    println!("");
    if clean_list_length == original_list_length {
        println!("I didn't find any problematic words. Your inputted word list appears to be compound-safe as is!");
    } else {
        println!(
            "The compound-safe list I made has {} words ({} bits per word). It's located at '{}'",
            clean_list_length,
            log_base(2, clean_list_length),
            &compound_safe_list_output
        );
    }
}

fn make_vec(filename: &str) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        word_list.push(l);
    }
    return word_list;
}

fn split_and_search(
    words: Vec<String>,
    word_list_to_check_filename: &str,
) -> (Vec<String>, Vec<Vec<String>>, Vec<String>) {
    let mut single_bad_words: Vec<String> = [].to_vec();
    let mut double_bad_words: Vec<Vec<String>> = [].to_vec();
    let mut must_remove_words: Vec<String> = [].to_vec();
    for mut word in words {
        println!("Starting search of {}", word);
        let mut second_half = "".to_string();
        for _i in 0..word.len() {
            let length = &word.len();
            second_half = format!("{}{}", &word.split_off(length - 1), second_half);
            if search(&word, word_list_to_check_filename) {
                // first check for compound-unsafe pairs
                println!("I found {} as its own word. second half is {} and I should search for that now", word, second_half);
                if search(&second_half, word_list_to_check_filename) {
                    single_bad_words.push(word.to_string());
                    single_bad_words.push(second_half.to_string());
                    double_bad_words.push(vec![word.to_string(), second_half.to_string()]);
                }
                // Now check for problematic overlapping words
                let overlap = &second_half; // boy
                for word in make_vec(word_list_to_check_filename) {
                    if overlap.len() < word.len() {
                        let overhang = &word[overlap.len()..word.len()]; // hood
                        if overlap == &word[0..overlap.len()].to_string()
                            && search(overhang, word_list_to_check_filename)
                        {
                            println!(
                                "word is {}, overlap is {}, and overhang is {}",
                                word, overlap, overhang
                            );
                            // must_remove_words.push(word.to_string());
                            must_remove_words.push(overhang.to_string());
                        }
                    }
                }
            }
        }
    }
    (single_bad_words, double_bad_words, must_remove_words)
}

fn search(target_word: &str, word_list_to_check_filename: &str) -> bool {
    let words = make_vec(&word_list_to_check_filename);
    for word in words {
        if target_word == word {
            return true;
        }
    }
    return false;
}

fn find_words_to_remove(
    single_bad_words: Vec<String>,
    double_bad_words: Vec<Vec<String>>,
    must_remove_words: Vec<String>,
) -> Vec<String> {
    let mut words_to_remove: Vec<String> = [].to_vec();
    for word_vec in double_bad_words {
        let mut first_word_appearances = 0;
        let mut second_word_appearances = 0;
        for word in &single_bad_words {
            if &word_vec[0] == word {
                first_word_appearances = first_word_appearances + 1;
            }
            if &word_vec[1] == word {
                second_word_appearances = second_word_appearances + 1;
            }
        }
        if first_word_appearances >= second_word_appearances {
            words_to_remove.push(word_vec[0].to_string());
        } else {
            words_to_remove.push(word_vec[1].to_string());
        }
    }

    for word in must_remove_words {
        words_to_remove.push(word.to_string());
    }

    words_to_remove.sort();
    words_to_remove.dedup();
    return words_to_remove;
}

fn make_clean_list(words_to_remove: Vec<String>, original_list: Vec<String>) -> Vec<String> {
    let mut clean_words: Vec<String> = [].to_vec();
    for original_word in original_list {
        let mut bad_word = false;
        for word_to_remove in &words_to_remove {
            if word_to_remove == &original_word {
                bad_word = true;
            }
        }
        if bad_word == false {
            clean_words.push(original_word);
        }
    }
    clean_words.sort();
    clean_words
}

fn log_base(base: u64, n: f64) -> f64 {
    let base_as_float: f64 = base as f64;
    return (n.ln() / base_as_float.ln()) as f64;
}

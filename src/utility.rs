pub fn purify_word(mut word: String) -> String {
    let not_allowed = vec!['[', ']', '"'];
    word.retain(|c| !not_allowed.contains(&c));
    word
}

pub fn print_to_guess(word: &String, tries: &mut Vec<char>) {

    let mut output: String = "".to_string();
    let mut new_string: String;

    for single in word.chars() {
        if !tries.contains(&single) {
            new_string = "_".to_string();
        }
        else {
            new_string = single.to_string();
        }
        output.push_str(&new_string);
        output.push_str(" ");
    }

    println!("{}", output);

}

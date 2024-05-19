use std::io::{stdin, stdout, Write};

fn ask() -> char {
    loop {
        print!("Letter to guess: ");
        stdout().flush().unwrap();

        let mut input_one = String::new();
        stdin().read_line(&mut input_one).unwrap();

        let _ = input_one.trim();
        input_one.pop();

        if input_one.len() == 1 {
            return input_one.chars().next().unwrap();
        } else {
            println!("Only one character");
        }
    }
}

fn find_position(input: String, word: String, target: char) -> String {
    let mut vecword = Vec::new();
    for n in word.chars() {
        vecword.push(n);
    }
    for (i, l) in input.chars().enumerate() {
        if l == target {
            vecword[i] = l;
        }
    }
    let wordresult = vecword.into_iter().collect::<String>();
    wordresult
}

fn main() {
    print!("Word to guess: ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("This is not a string");
    input.pop();

    let mut guessword = String::new();
    for _n in 0..input.len() {
        guessword.push('_');
    }

    loop {
        let lettertoguess = ask();
        guessword = find_position(input.to_lowercase().clone(), guessword, lettertoguess);
        println!("{}", guessword);
        if input.to_lowercase() == guessword {
            break;
        }
    }
    println!("You guessed the word it was: {input}");
}

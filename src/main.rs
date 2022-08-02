use std::io;

fn main() {
    let alphabet: Vec<char> = ('a'..='z').collect();

    let mut read_offset = String::new();
    let mut read_phrase = String::new();

    println!("Please provide the offset");
    io::stdin().read_line(&mut read_offset).unwrap();
    let offset: i32 = read_offset.trim().parse().unwrap();

    println!("Please provide your phrase");
    io::stdin().read_line(&mut read_phrase).unwrap();

    let new_phrase = cipher(read_phrase, alphabet.clone(), offset.try_into().unwrap());
    println!("Cipher: {:?}", new_phrase);
    let old_phrase = decipher(new_phrase, alphabet, offset.try_into().unwrap());
    println!("Decipher: {:?}", old_phrase)
}

fn cipher(phrase: String, alphabet: Vec<char>, offset: usize) -> String {
    let mut new_phrase = String::new();
    for c in phrase.trim().chars() {
        if c == ' ' {
            new_phrase.push(' ')
        } else {
            let index = alphabet.iter().position(|&r| r == c).unwrap();
            let at_index = index + offset;
            let current = alphabet[at_index];
            new_phrase.push(current)
        }
    }
    new_phrase
}

fn decipher(cipher: String, alphabet: Vec<char>, offset: usize) -> String {
    let mut new_phrase = String::new();
    for c in cipher.trim().chars() {
        if c == ' ' {
            new_phrase.push(' ')
        } else {
            let index = alphabet.iter().position(|&r| r == c).unwrap();
            let at_index = index - offset;
            let current = alphabet[at_index];
            new_phrase.push(current)
        }
    }
    new_phrase
}

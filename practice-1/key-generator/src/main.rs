use solana_sdk::signer::{keypair::Keypair, Signer};
use std::{self, thread};

fn main() {
    let mut user_input = String::from("");
    const ATTEMPT_AMOUNT: i32 = 10000000; // 1 mil attempts to create a key
    println!("Enter the prefix you want to have in your public key (length should be greater than or equal 1 and less than or equal 6):");
    std::io::stdin().read_line(&mut user_input).unwrap();
    trim_newline(&mut user_input);

    if user_input.chars().count() < 1 || user_input.chars().count() > 6 {
        println!("You printed prefix with incorrect length: length should be greater than or equal 1 and less than or equal 6. {}", user_input.chars().count());
        return;
    } else {
            let uset_input_copy = user_input.clone();
            let handle = thread::spawn(move || {
                generate_key_with_prefix(ATTEMPT_AMOUNT, uset_input_copy);
            });
            generate_key_with_prefix(ATTEMPT_AMOUNT, user_input);
            handle.join();
        }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn display_bytes(bytes: &[u8]) -> String {
    let mut result = String::from("[");
    for (index, byte) in bytes.iter().enumerate() {
        let next_symbol = if index >= bytes.len() - 1 { "]" } else {", "};
        result += &byte.to_string();
        result += &String::from(next_symbol);
    };
    
    return result;
}

fn generate_key_with_prefix(attempts_limit: i32, user_input: String) {
    println!("{}", user_input);

    let mut counter = 0;
    let mut keypair;
    while counter  != attempts_limit {
        keypair = Keypair::new();
        if keypair.pubkey().to_string().starts_with(&user_input) {
            println!("The key is successfully generated!");
            println!("The private key: {}", display_bytes(keypair.secret().as_bytes()));
            println!("The public key:{}", keypair.pubkey());
            break;
        }

        println!("{}, {}", counter, keypair.pubkey());
        counter += 1;
    }

    if counter == attempts_limit {
        println!("Unfortunately key with such prefix wasn't generated during {} attemtps. Please, try again.", attempts_limit);
    }
}


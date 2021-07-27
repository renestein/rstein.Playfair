mod globalconstants;
mod maintest;
mod operationtype;
mod playfairconfiguration;

use globalconstants::*;
use operationtype::*;
use playfairconfiguration::PlayfairConfiguration;
use std::collections;
use std::io;

fn main() {
    let mut config = PlayfairConfiguration::new();
    println!("{}", config);

    loop {
        let mut choice = String::new();
        println!("1) Encrypt data.");
        println!("2) Decrypt data.");
        let input = io::stdin().read_line(&mut choice);
        if input.is_err() {
            println!("Invalid choice. Try again.");
            continue;
        }

        config.operation_type = match choice.chars().enumerate().next() {
            Some((_, '1')) => OperationType::Encrypt,
            Some((_, '2')) => OperationType::Decrypt,
            Some((_, ichar)) => {
                println!("Invalid choice {0}. Ty again.", ichar);
                continue;
            }
            None => {
                println!("Invalid choice. Try again.");
                continue;
            }
        };

        break;
    }

    println!("Input your key (max 25 chars will be used) and press <Enter>:");
    loop {
        let read_key_result = io::stdin().read_line(&mut config.key);
        match read_key_result {
            Ok(_) => config.key = config.key.trim_end().to_uppercase(),
            Err(error) => {
                println!("An error occured while reading key: {}", error);
                continue;
            }
        };

        break;
    }

    let mut raw_input_text = String::new();
    if config.is_encrypt() {
        println!("Input your plain text and press <Enter>:");
    } else {
        println!("Input your cipher text and press <Enter>:");
    }

    let read_result = io::stdin().read_line(&mut raw_input_text);
    match read_result {
        Ok(_) => {
            raw_input_text = raw_input_text.to_uppercase();
        }

        Err(error) => panic!("Unexpected error while reading input data {}.", error),
    }

    let output_text = encrypt_decrypt(&raw_input_text, &config);
    println!("Output text:");
    println!("");
    println!("{}", output_text);
}

pub fn encrypt_decrypt(raw_input_text: &str, config: &PlayfairConfiguration) -> String {
    let (key_table, ascii_table) = generate_key_table(config);
    let input_text = prepare_input_text(raw_input_text, &ascii_table, config);
    let mut output_text = String::with_capacity(input_text.len());
    let mut first_char = config.unused_char;

    for input_char in input_text.chars() {
        if first_char == config.unused_char {
            first_char = input_char;
            continue;
        }

        let first_char_row = key_table
            .iter()
            .position(|row| row.contains(&first_char))
            .unwrap();
        let first_char_column = key_table[first_char_row]
            .iter()
            .position(|c| first_char == *c)
            .unwrap();
        let second_char_row = key_table
            .iter()
            .position(|row| row.contains(&input_char))
            .unwrap();
        let second_char_column = key_table[second_char_row]
            .iter()
            .position(|c| input_char == *c)
            .unwrap();

        let add_to_same_row_or_column = {
            if config.is_decrypt() {
                KEY_ROWS - 1
            } else {
                1
            }
        };

        match (
            first_char_row == second_char_row,
            first_char_column == second_char_column,
        ) {
            (true, false) => {
                output_text.push(
                    key_table[first_char_row]
                        [(first_char_column + add_to_same_row_or_column) % KEY_COLUMNS],
                );
                output_text.push(
                    key_table[second_char_row]
                        [(second_char_column + add_to_same_row_or_column) % KEY_COLUMNS],
                )
            }
            (false, true) => {
                output_text.push(
                    key_table[(first_char_row + add_to_same_row_or_column) % KEY_ROWS]
                        [first_char_column],
                );
                output_text.push(
                    key_table[(second_char_row + add_to_same_row_or_column) % KEY_ROWS]
                        [(second_char_column)],
                );
            }
            (_, _) => {
                output_text.push(key_table[first_char_row][second_char_column]);
                output_text.push(key_table[second_char_row][first_char_column]);
            }
        }

        first_char = config.unused_char;
    }

    return output_text;
}

fn prepare_input_text(
    raw_input_text: &str,
    ascii_table: &[char],
    config: &PlayfairConfiguration,
) -> String {
    let mut output_text = String::with_capacity(raw_input_text.len() + 1);
    let mut first_char = config.unused_char;
    for elem in raw_input_text.to_uppercase().chars() {
        if !ascii_table.contains(&elem) && elem != config.unused_char {
            continue;
        }

        if config.is_decrypt() {
            output_text.push(elem);
            continue;
        }

        let plain_text_char = {
            if elem == config.unused_char {
                config.replace_char_for_unused_char
            } else {
                elem
            }
        };

        if first_char != config.unused_char {
            output_text.push(first_char);
            if first_char == plain_text_char {
                output_text.push(config.surrogate_char);
            }
            output_text.push(plain_text_char);
            first_char = config.unused_char;
        } else {
            first_char = plain_text_char;
        }
    }

    if config.is_encrypt() {
        if first_char != config.unused_char {
            output_text.push(first_char);
            output_text.push(config.surrogate_char);
        }
    }

    return output_text;
}

fn generate_key_table(config: &PlayfairConfiguration) -> (Vec<Vec<char>>, Vec<char>) {
    let mut ascii_table = get_ascii_table();
    ascii_table.retain(|c| *c != config.unused_char);
    print_char_table(&ascii_table);

    let mut key_table = vec![vec!['0'; KEY_COLUMNS]; KEY_ROWS];
    let mut current_key_table_index_0 = 0;
    let mut current_key_table_index_1 = 0;
    let mut used_chars: collections::HashSet<char> = collections::HashSet::new();

    for key_char in config.key.chars() {
        if used_chars.contains(&key_char) {
            continue;
        }

        if !ascii_table.contains(&key_char) {
            continue;
        }

        used_chars.insert(key_char);
        key_table[current_key_table_index_0][current_key_table_index_1] = key_char;
        current_key_table_index_1 += 1;
        if current_key_table_index_1 == KEY_COLUMNS {
            current_key_table_index_0 += 1;

            if current_key_table_index_0 == KEY_ROWS {
                break;
            }

            current_key_table_index_1 = 0;
        }
    }

    for row_index in current_key_table_index_0..KEY_ROWS {
        for column_index in current_key_table_index_1..KEY_COLUMNS {
            let unused_ascii_letter = *ascii_table
                .iter()
                .find(|al| !used_chars.contains(&al))
                .unwrap();

            key_table[row_index][column_index] = unused_ascii_letter;
            used_chars.insert(unused_ascii_letter);
            current_key_table_index_1 = 0;
        }
    }

    for row in 0..KEY_ROWS {
        println!("Key row {0}", row);
        print_char_table(&key_table[row]);
    }

    (key_table, ascii_table)
}

fn get_ascii_table() -> Vec<char> {
    const ASCII_START: usize = 65;
    const ASCII_LENGTH: usize = 26;
    let mut ascii_array: Vec<char> = Vec::new();

    for i in ASCII_START..ASCII_START + ASCII_LENGTH {
        let ascii_char = i as u8 as char;
        ascii_array.push(ascii_char);
    }

    return ascii_array;
}

fn print_char_table(table: &[char]) {
    for c in table.iter() {
        println!("{}", c);
    }
}

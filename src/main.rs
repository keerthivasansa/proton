mod functions;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn regex(s: &str) -> Regex {
    Regex::new(s).expect("Failed to compile the regex")
}

enum Heap<'a> {
    Int(i32),
    Str(&'a str),
    Bool(bool),
}

struct Context<'a> {
    variable_map: HashMap<&'a str, Heap<'a>>,
}

impl Context<'_> {
    fn set_var(&mut self, var_name: &'static str, val: Heap<'static>) {
        self.variable_map.insert(var_name.clone(), val);
    }

    fn get_var(&self, var_name: &'static str) -> Option<Heap> {
        if let Some(heap) = self.variable_map.get(var_name) {
            let val = match heap {
                &Heap::Int(i) => Heap::Int(i),
                &Heap::Bool(b) => Heap::Bool(b),
                &Heap::Str(s) => Heap::Str(s),
            };
            Some(val)
        } else {
            None
        }
    }

    fn new() -> Context<'static> {
        return Context {
            variable_map: HashMap::new(),
        };
    }
}

fn copy_heap(h:Heap) -> Heap {
    match h {
        Heap::Int(b) => Heap::Int(b),
        Heap::Str(b) => Heap::Str(b),
        Heap::Bool(b) => Heap::Bool(b),
    }
}

// returns a string from the sequence of characters
fn get_string(chars: &Vec<char>, mut char_index: usize) -> (String, usize) {
    let mut strd = String::new();
    char_index += 1; // skip the leading double quote
    let mut letter = chars[char_index];
    while letter != '"' && char_index < chars.len() - 1 {
        strd.push(letter);
        char_index += 1;
        letter = chars[char_index];
    }
    char_index += 1; // consume the trailing double quote
    return (strd, char_index);
}

fn main() {
    let mut global_ctx = Context::new();
    let args: Vec<String> = env::args().collect();
    let fn_call = regex(r"([\w]+)\((.*)");
    let no_args = args.len();
    if no_args < 2 {
        panic!("No filename was provided");
    }
    let filename = &args[1];
    println!("Opening the file: {filename}");
    let file_content = fs::read_to_string(filename).expect("Failed to read the file");

    let chars: Vec<char> = file_content.chars().collect();
    let length = chars.len();
    let mut char_index = 0;

    while char_index < length {
        let mut letter = chars[char_index];

        if letter == '/' {
            // remove the comments
            char_index += 1;
            let next_letter = chars[char_index];
            if next_letter != '/' {
                panic!("Unexpected token. Expected: '/'")
            }
            while letter != '\n' {
                char_index += 1;
                letter = chars[char_index];
            }
        }

        if letter.is_alphabetic() {
            // identifier / keyword found
            let mut token = String::new();

            while !letter.is_whitespace() && char_index < length - 1 {
                token.push(chars[char_index]);
                char_index += 1;
                letter = chars[char_index];
            }

            if fn_call.is_match(&token) {
                while letter != ')' && char_index < length - 1 {
                    if letter == '"' {
                        let (mut str, new_char_index) = get_string(&chars, char_index);
                        str.insert(0, '"');
                        char_index = new_char_index - 1;
                        token += str.as_str();
                        continue;
                    }

                    char_index += 1;
                    letter = chars[char_index];
                    if letter != '"' {
                        token.push(letter);
                    }
                }

                let token_chars: Vec<char> = token.chars().collect();
                let mut token_index = 0;
                let mut token_letter = token_chars[token_index];
                let mut fn_name = String::new();

                fn_name.push(token_letter);

                while token_chars[token_index + 1] != '(' {
                    token_index += 1;
                    token_letter = token_chars[token_index];
                    fn_name.push(token_letter);
                }

                let fn_args: Vec<&str> = token[token_index + 2..token.trim().len() - 1]
                    .split(',')
                    .collect(); // remove the fn_name, brackets and trailing whitespaces

                println!("{fn_name} fn called with args:\n {}", fn_args.join(", "));
            }
        }

        char_index += 1;
    }
}

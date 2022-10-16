fn main2() {
    let mut int_val: Vec<i32> = vec![];
    let mut str_map: Vec<String> = vec![];
    let mut heap_map = HashMap::new();

    let operators_regex = regex(r"^[*-+/{}()=]$");
    let keyword_regex = regex(r"^if|else|elif|return|for|let|const|from|to|till$");
    let fn_call = regex(r"^([\w]+)\(([\w\W]*)\)$");
    let identifier_regex = regex(r"\w\W*");
    let token_splitter = regex(r"\s|;");
    let line_split = regex("\n");
    let numbers_regex = regex("[0-9]*");

    let args: Vec<String> = env::args().collect();
    let no_args = args.len();
    if no_args < 2 {
        println!("No filename was provided");
        return;
    }
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Failed to read the file: {filename}");
    let lines: Vec<&str> = line_split
        .split(&content)
        .filter(|x| x.trim() != "")
        .collect();

    let mut current_token = "";

    let for_regex = Regex::new("^kikiki").expect("Failed to compile the regex for for loop");

    for (line_index, line) in lines.iter().enumerate() {
        let line_no = line_index + 1; // to account for difference between numbers and index
                                      // println!("{line}");
        let mut tokens: Vec<&str> = token_splitter
            .split(line)
            .map(|x| x.trim())
            .filter(|&x| x != "")
            .collect();

        // remove comment lines
        if let Some(index) = tokens.iter().position(|x| x.starts_with("//")) {
            tokens.drain(index..);
        };

        let mut pattern: Vec<Token> = vec![];

        for tok in tokens.iter() {
            if DATA_TYPES.contains(&tok) {
                pattern.push(Token::DataType);
            } else if keyword_regex.is_match(tok) {
                pattern.push(Token::Keyword)
            } else if identifier_regex.is_match(tok) {
                pattern.push(Token::Identifier);
            } else if operators_regex.is_match(tok) {
                pattern.push(Token::Operator)
            }
        }

        let pattern_strings: Vec<&str> = pattern.iter().map(|x| x.as_str()).collect();
        let pattern_signature = pattern_strings.join("");
        let is_for = for_regex.is_match(&pattern_signature);

        if pattern.len() == 0 {
            continue;
        }

        if pattern[0] == Token::DataType {
            // Declaration of variable / function / parameter
            let token = tokens[1];
            if pattern[1] != Token::Identifier {
                println!("\nUnexpected symbol: {token} in line no: {line_no}\n");
                panic!()
            }
            let op = tokens[2];
            if pattern[2] == Token::Operator {
                if op == "(" || op.replace(" ", "") == "()" {
                } else if op == "=" {
                    if tokens[3].starts_with('"') {
                        let get_last_pos =
                            ((tokens[3..]).iter().position(|x| x.ends_with('"')).expect(
                                &("Unclosed string at line:".to_owned() + &line_no.to_string()),
                            )) + 3
                                + 1;
                        let literal_tokens: Vec<&str> = tokens.drain(3..get_last_pos).collect();
                        let val = literal_tokens.join(" ").replace('"', "");
                        // assign the value to the name
                        str_map.push(val);
                        let index = str_map.len() - 1;
                        heap_map.insert(token, (Heap::Str, index));
                    } else if numbers_regex.is_match(tokens[3]) {
                        // it is a number
                        int_val.push(
                            tokens[3]
                                .parse()
                                .expect("Unable to parse the number at line"),
                        );
                        let index = int_val.len() - 1;
                        heap_map.insert(token, (Heap::Int, index));
                    };
                }
            }
        }

        if pattern[0] == Token::Identifier {
            if fn_call.is_match(tokens[0]) {
                let matches = fn_call.captures(tokens[0]);
                if let Some(m) = matches {
                    let fn_name = m.get(1).expect("Failed to get fn name").as_str();
                    let args: Vec<&str> = m
                        .get(2)
                        .expect("Failed to get fn args")
                        .as_str()
                        .split(",")
                        .collect();
                    match fn_name {
                        "print" => {
                            let args_: Vec<String> = args
                                .iter()
                                .map(|&x| {
                                    if !x.starts_with('"') && !x.ends_with('"') {
                                        // it is a variable
                                        let var_name_arg = x.replace("'", "");
                                        let var_name = var_name_arg.as_str();
                                        if let Some((map_type, index)) = heap_map.get(var_name) {
                                            match *map_type {
                                                Heap::Int => int_val[*index].to_string(),
                                                Heap::Str => {
                                                    str_map.get(*index).expect("Failed").to_owned()
                                                }
                                            }
                                        } else {
                                            panic!("Failed to find the variable");
                                        }
                                    } else {
                                        let r = regex("[\"']").replace_all(x, "").to_owned();
                                        String::from(r)
                                    }
                                })
                                .collect();
                            print!("{}", args_.join(" "));
                        }
                        _ => (),
                    }
                }
            }
        }

        if pattern[0] == Token::Keyword {
            match tokens[0] {
                "for" => {
                    if is_for {
                        let start = match (tokens[3]).parse::<i32>() {
                            Ok(a) => a,
                            _ => -1,
                        };

                        let stop_n = match (tokens[5]).parse::<i32>() {
                            Ok(a) => a,
                            _ => -1,
                        };

                        let stop = if tokens[4] == "till" {
                            stop_n + 1
                        } else if tokens[4] == "to" {
                            stop_n
                        } else {
                            -1
                        };

                        for i in start..stop {
                            print!("{i}");
                        }
                    };
                }
                _ => (),
            };
        }
    }
}

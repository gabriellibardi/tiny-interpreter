use std::{collections::HashMap, vec};

fn main() {
    let program = r#"
        var a
        func f() {
            a = 5
            var b
            b = 6
        }
        func g() {
            var c
            c = 7
            f()
        }
        g()
    "#;

    let mut global_symbol_table: HashMap<String, usize> = HashMap::new();
    println!("Analysis:");
    let mut function_table = analyze(program, &mut global_symbol_table);
    println!("\nExecution:");
    execute(program, &mut global_symbol_table, &mut function_table);
}

fn analyze(program: &str, global_symbol_table: &mut HashMap<String, usize>) -> HashMap<String, usize> {
    let mut function_table: HashMap<String, usize> = HashMap::new();
    let mut local_symbol_table: Vec<String> = Vec::new();
    let mut memory_address = 0;
    let mut is_local = false;

    for (line_number, line) in program.trim().lines().enumerate() {
        let line = line.trim();
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens.as_slice() {
            ["var", name] => {
                if is_local {
                    if local_symbol_table.contains(&name.to_string())
                        || global_symbol_table.contains_key(*name)
                    {
                        println!("variable redefined: {}", name);
                    } else {
                        local_symbol_table.push(name.to_string());
                    }
                } else {
                    if global_symbol_table.contains_key(*name) {
                        println!("variable redefined: {}", name);
                    } else {
                        global_symbol_table.insert(name.to_string(), memory_address);
                        memory_address += 1;
                    }
                }
            }
            [name, "=", _number] => {
                if is_local {
                    if !local_symbol_table.contains(&name.to_string())
                        && !global_symbol_table.contains_key(*name)
                    {
                        println!("variable unknown: {}", name);
                    }
                } else if !global_symbol_table.contains_key(*name) {
                    println!("variable unknown: {}", name);
                }
            }
            ["func", name, "{"] => {
                is_local = true;
                local_symbol_table.clear();
                if function_table.contains_key(*name) {
                    println!("function redefined: {}", name);
                } else {
                    function_table.insert(name.to_string(), line_number);
                }
            }
            ["}"] => {
                if !local_symbol_table.is_empty() {
                    println!(
                        "clearing local_symbol_table: {:?}",
                        local_symbol_table
                    );
                }
                is_local = false;
            }
            [name] if name.ends_with("()") => {
                if !function_table.contains_key(*name) {
                    println!("function unknown: {}", name);
                }
            }
            _ => {
                println!("unmatched: {}", line);
            }
        }
    }
    local_symbol_table.clear();

    println!("analysis ended\n");
    println!("global_symbol_table: {:?}", global_symbol_table);
    println!("local_symbol_table: {:?}", local_symbol_table);
    println!("function_table: {:?}", function_table);
    function_table
}

fn execute(program: &str, global_symbol_table: &mut HashMap<String, usize>, function_table: &mut HashMap<String, usize>) {
    let mut pc = 0;
    let mut call_stack: Vec<usize> = Vec::new();
    let mut activation_frames: Vec<HashMap<String, usize>> = Vec::new();
    let mut memory: Vec<i32> = vec![0; global_symbol_table.len()];
    let mut is_local = false;
    let mut memory_address = memory.len();
    let lines: Vec<&str> = program.trim().lines().collect();

    while pc < lines.len() {
        let line: Vec<&str> = lines[pc].split_whitespace().collect();
        match line.as_slice() {
            ["var", name] => {
                if is_local && !global_symbol_table.contains_key(*name) {
                    memory.push(0);
                    match activation_frames.last_mut() {
                        Some(frame) => {
                            frame.insert(name.to_string(), memory_address);
                        }
                        None => println!("no activation frame available")
                    }
                    println!("created local {name} with address {memory_address}");
                    memory_address += 1;
                }
            }
            [name, "=", number] => {
                let address = if is_local && !global_symbol_table.contains_key(*name) {
                    activation_frames
                        .last()
                        .and_then(|frame| frame.get(*name))
                        .copied()
                }
                else {
                    global_symbol_table.get(*name).copied()
                };
                match address {
                    Some(value) => {
                        match number.parse::<i32>() {
                            Ok(n) => {
                                memory[value] = n;
                                println!("{name} at address {value} receives {n}")
                            }
                            Err(e) => println!("Error converting {number}: {e} "),
                        }
                    }
                    None => {
                        println!("Unknown variable: {name}");
                    }
                }
            }
            ["func", _name, "{"] => {
                while lines[pc].trim() != "}" {
                    pc += 1
                }
            }
            [name] if name.ends_with("()") => {
                activation_frames.push(HashMap::new());
                call_stack.push(pc);
                println!("{name} called in line {pc}");
                match function_table.get(*name) {
                    Some(function) => pc = *function,
                    None => println!("invalid function"),
                }
                is_local = true;
            }
            ["}"] => {
                is_local = false;
                println!("memory before removal of local variables: {:?}", memory);  
                match activation_frames.pop() {
                    Some(frame) => {
                        println!("deleting last activation frame: {:?}", frame);
                        memory.truncate(memory.len() - frame.len());
                        match call_stack.pop() {
                            Some(value) => {
                                pc = value;
                                println!("return to line {pc}")
                            } 
                            None => println!("empty call stack"),
                        }
                    }
                    None => println!("invalid frame"),
                }
            }
            _ => println!("invalid line"),
        }
        pc += 1
    }
    println!("execution ended\n");
    println!("memory: {:?}", memory);
    println!("call_stack: {:?}", call_stack);
    println!("activation frames: {:?}", activation_frames);
}

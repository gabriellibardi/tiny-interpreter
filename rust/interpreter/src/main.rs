use std::collections::HashMap;

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
    analyze(program, &mut global_symbol_table);
}

fn analyze(program: &str, global_symbol_table: &mut HashMap<String, usize>) {
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
                        println!("[error] variable redefined: {}", name);
                    } else {
                        local_symbol_table.push(name.to_string());
                    }
                } else {
                    if global_symbol_table.contains_key(*name) {
                        println!("[error] variable redefined: {}", name);
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
                        println!("[error] variable unknown: {}", name);
                    }
                } else if !global_symbol_table.contains_key(*name) {
                    println!("[error] variable unknown: {}", name);
                }
            }
            ["func", name, "{"] => {
                is_local = true;
                local_symbol_table.clear();
                if function_table.contains_key(*name) {
                    println!("[error] function redefined: {}", name);
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
                    println!("[error] function unknown: {}", name);
                }
            }
            _ => {
                println!("[error] unmatched: {}", line);
            }
        }
    }
    local_symbol_table.clear();

    println!("analysis ended\n");
    println!("global_symbol_table: {:?}", global_symbol_table);
    println!("local_symbol_table: {:?}", local_symbol_table);
    println!("function_table: {:?}", function_table);
}

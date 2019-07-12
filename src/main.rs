use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

#[derive(Debug)]
struct Context {
    vars: HashMap<String, i64>,
}
impl Context {
    fn new() -> Context {
        let hm = HashMap::new();
        Context{vars: hm}
    }
    fn process<'a>(&'a mut self, line: String) {
        let line = line.trim();
        if line.starts_with("\\") {
            let words: Vec<&str> = line.split(' ').collect();
            match words[0] {
                "\\init" => {
                    let key = words[1].to_string();
                    let value: i64 = words[2].parse().unwrap();
                    self.vars.insert(key, value);
                },
                "\\incr" => {
                    let key = words[1].to_string();
                    let value: i64 = words[2].parse().unwrap();
                    *self.vars.get_mut(&key).unwrap() += value;
                },
                _ => {
                    println!("[literal] unknown directive: {}", line);
                }
            }
        } else {
            println!("{}", line);
        }

    }
} 

fn main() -> Result<()> {
    let file = File::open("examples/basic.in.txt")?;
    let mut ctx = Context::new();
    for line in BufReader::new(file).lines() {
        ctx.process(line?);
    }
    println!("\n\n{:#?}", ctx);
    Ok(())
}

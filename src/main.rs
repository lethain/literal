use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::path::Path;

extern crate tera;

#[derive(Debug)]
struct Context {
    vars: HashMap<String, i64>,
}
impl Context {
    fn new() -> Context {
        let hm = HashMap::new();
        Context{vars: hm}
    }

    fn render_template(&self, path: String) -> Result<String, String> {
        let path: &Path = Path::new(&path);
        let dir = path.parent().unwrap().join("*").to_str().unwrap().to_string();
        let filename = path.file_name().unwrap().to_str().unwrap();

        let mut tera_ctx = tera::Context::new();
        //tera_ctx.insert("age", &18);
        for (key, &val) in self.vars.iter() {
            tera_ctx.insert(key, &val);
        }

        match tera::Tera::new(&dir) {
            Ok(t) => {
                match t.render(filename, &tera_ctx) {
                    Ok(rendered) => Ok(rendered.as_str().to_string()),
                    Err(e) => Err(e.to_string()),
                }
            },
            Err(e) => {
                let err_str = format!("[literal] Failed loading templates: {}", e);
                Err(err_str.to_string())
            }            
        }
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
                    match self.vars.get_mut(&key) {
                        Some(current) => *current += value,
                        None => eprintln!("[literal] incremented non-existant variable: {}", line),
                    }
                },
                "\\render" => {
                    let filename = words[1].to_string();
                    match self.render_template(filename) {
                        Ok(rendered) => println!("{}", rendered),
                        Err(e) => eprintln!("[literal] error rendering: {}", e),
                    }
                    
                },
                _ => {
                    eprintln!("[literal] unknown directive: {}", line);
                }
            }
        } else {
            println!("{}", line);
        }
    }
} 

fn main() -> std::io::Result<()> {
    let file = File::open("examples/basic.in.txt")?;
    let mut ctx = Context::new();
    for line in BufReader::new(file).lines() {
        ctx.process(line?);
    }
    Ok(())
}

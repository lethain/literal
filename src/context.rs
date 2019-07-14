extern crate tera;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct Context {
    vars: HashMap<String, i64>,
}

impl Context {
    pub fn new() -> Context {
        let hm = HashMap::new();
        Context{vars: hm}
    }
    
    pub fn render(&mut self, file: File) -> Result<String, String> {
        let mut acc: String = "".to_string();
        for line in BufReader::new(file).lines() {
            match line {
                Ok(line) => {
                    match self.process(line) {
                        Ok(rendered) => acc = acc.to_string() + &rendered,
                        Err(e) => return Err(e.to_string()),                
                    }
                },
                Err(e) => return Err(e.to_string()),
            }
        };
        Ok(acc.to_string())
    }

    fn process<'a>(&'a mut self, line: String) -> Result<String, String> {
        let line = line.trim();
        if line.starts_with("\\") {
            let words: Vec<&str> = line.split(' ').collect();
            match words[0] {
                "\\init" => self.directive_init(words),
                "\\incr" => self.directive_incr(line.to_string(), words),
                "\\render" => {
                    let filename = words[1].to_string();
                    match self.render_template(filename) {
                        Ok(rendered) => Ok(rendered),
                        Err(e) => Err(format!("[literal] error rendering: {}", e).to_string())
                    }
                    
                },
                _ => {
                    Err(format!("[literal] unknown directive: {}", line).to_string())
                }
            }
        } else {
            Ok(line.to_string()+"\n")
        }
    }

    fn directive_init(&mut self, words: Vec<&str>) -> Result<String, String> {
        let key = words[1].to_string();
        let value: i64 = words[2].parse().unwrap();
        self.vars.insert(key, value);
        Ok("".to_string())
    }

    fn directive_incr(&mut self, line: String, words: Vec<&str>) -> Result<String, String> {
        let key = words[1].to_string();
        let value: i64 = words[2].parse().unwrap();
        match self.vars.get_mut(&key) {
            Some(current) => {
                *current += value;
                Ok("".to_string())
            },
            None => Err(format!("[literal] incremented non-existant variable: {}", line).to_string())
        }
    }


    fn render_template(&self, path: String) -> Result<String, String> {
        let path: &Path = Path::new(&path);
        let dir = path.parent().unwrap().join("*").to_str().unwrap().to_string();
        let filename = path.file_name().unwrap().to_str().unwrap();

        let mut tera_ctx = tera::Context::new();
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

} 
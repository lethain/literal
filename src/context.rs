extern crate tera;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
enum VariableType {
    IntegerVariable,
    StringVariable,
}

#[derive(Debug)]
struct Variable {
    vt: VariableType,
    iv: i64,
    sv: String,
}

impl Variable {
    fn integer(integer: i64) -> Variable {
        Variable{vt: VariableType::IntegerVariable, iv: integer, sv: "".to_string()}
    }
    fn string(string: String) -> Variable {
        Variable{vt: VariableType::StringVariable, iv:0, sv: string}
    }
}

#[derive(Debug)]
pub struct Context {
    vars: HashMap<String, Variable>,
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

    fn process(&mut self, line: String) -> Result<String, String> {
        let line = line.trim();
        if line.starts_with("\\") {
            let words: Vec<&str> = line.split(' ').collect();
            match words[0] {
                "\\init" => self.directive_init(words),
                "\\incr" => self.directive_incr(line.to_string(), words),
                "\\render" => self.directive_render(words),
                "\\assert" => self.directive_assert(words),
                _ => Err(format!("[literal] unknown directive: {}", line).to_string()),
            }
        } else {
            Ok(line.to_string()+"\n")
        }
    }
    fn directive_render(&self, words: Vec<&str>) -> Result<String, String> {
        let filename = words[1].to_string();
        match self.render_template(filename) {
            Ok(rendered) => Ok(rendered),
            Err(e) => Err(format!("[literal] error rendering: {}", e).to_string())
        }
    }

    fn directive_assert(&self, words: Vec<&str>) -> Result<String, String> {
        let key = words[1].to_string();
        let actual = self.vars.get(&key).unwrap();
        match actual.vt {
            VariableType::IntegerVariable => {
                let expected: i64 = words[2].parse().unwrap();
                if actual.iv == expected {
                    Ok("".to_string())
                } else {
                    Err(format!("[literal] expected {} to be {} but was {}", key, expected, actual.iv).to_string())
                }
            },
            VariableType::StringVariable => {
                let expected: String = words[2].to_string();
                if actual.sv == expected {
                    Ok("".to_string())
                } else {
                    Err(format!("[literal] expected {} to be {} but was {}", key, expected, actual.sv).to_string())
                }
            },
        }
    }

    fn directive_init(&mut self, words: Vec<&str>) -> Result<String, String> {
        let key = words[1].to_string();
        let word: &str = words[2];

        if self.vars.contains_key(&key) {
            let err_msg = format!("already initialized {}", key);
            return Err(err_msg.to_string())
        }

        match word.parse::<i64>() {
            Ok(value) => {
                let var = Variable::integer(value);
                self.vars.insert(key, var);
                Ok("".to_string())
            },
            Err(_) => {
                let acc = words[2..].join(" ");
                let var = Variable::string(acc.to_string());
                self.vars.insert(key, var);
                Ok("".to_string())
            }
        }
    }

    fn directive_incr(&mut self, line: String, words: Vec<&str>) -> Result<String, String> {
        let key = words[1].to_string();
        match self.vars.get_mut(&key) {
            Some(current) => {
                match current.vt {
                    VariableType::IntegerVariable => {
                        let value: i64 = words[2].parse().unwrap();
                        let var = Variable::integer(current.iv+value);
                        self.vars.insert(key, var)
                    },
                    VariableType::StringVariable => {
                        let acc = words[2..].join(" ");
                        let var = Variable::string(acc);
                        self.vars.insert(key, var)        
                    },
                };
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
        for (key, val) in self.vars.iter() {
            match val.vt {
                VariableType::IntegerVariable => tera_ctx.insert(key, &val.iv),
                VariableType::StringVariable => tera_ctx.insert(key, &val.sv),
            }
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
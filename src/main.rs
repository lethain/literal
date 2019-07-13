use std::env;
use std::fs::File;

mod context;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "must specify a file to render")),
        _ => {
            let file = File::open(&args[1])?;
            match context::Context::new().render(file) {
                Ok(rendered) => {
                    println!("{}", rendered);
                    Ok(())
                },
                Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
            }
        },
    }
}

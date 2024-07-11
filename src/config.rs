use std::env;
pub struct Config {
    pub file_directory: String,
}

impl Config {
    pub fn new_from_env() -> Result<Config, &'static str> {
        let mut env_args = env::args().skip(1);
        let mut file_directory: Option<String> = None;
        while let Some(arg) = env_args.next() {
            match &arg[..] {
                "--directory" => {
                    if let Some(d) = env_args.next() {
                        file_directory = Some(d);
                    } else {
                        return Err("No value specified for parameter --directory");
                    }
                }
                _ => {
                    if arg.starts_with('-') {
                        println!("Unkown argument {}", arg);
                    } else {
                        println!("Unkown positional argument {}", arg);
                    }
                }
            }
        }
        Ok(Config {
            file_directory: file_directory.ok_or("No file directory specified")?,
        })
    }
}

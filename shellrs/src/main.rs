use std::io::{self, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::process::Command;
use std::error::Error;
use std::env;
use std::path::Path;
use home;
//mod parser;

enum Builtin {
    Echo,
    History,
    Cd,
    Pwd,
    Exit
}


impl FromStr for Builtin {
    type Err = ();
    fn from_str(s : &str) -> Result<Self, Self::Err> {
      match s {
        "echo" => Ok(Builtin::Echo),
        "history" => Ok(Builtin::History),
        "cd" => Ok(Builtin::Cd),
        "pwd" => Ok(Builtin::Pwd),
        "exit" => Ok(Builtin::Exit),
        _ => Err(()),
      }
    }
}

static PROMPT_CHAR: char = '>';

fn main() {
    
    match get_system_info() {
        Ok((username, host, home_dir)) => {
            loop {
                // Print the shell prompt
                
        
                print_prompt(&username, &host, &home_dir);
        
                let input = read_command();
                let input = input.trim().to_string();
                
                if input == "exit" {
                    break;
                }
        
                let (command, args) = tokenize_command(input);
        
                process_command(command, args);
        
        
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    
    
}

fn get_system_info() -> Result<(String, String, PathBuf), Box<dyn Error>> {
    let username = users::get_current_username()
        .ok_or("Failed")?
        .into_string()
        .map_err(|_| "Failed to convert username to string")?;
    let host = hostname::get()?
        .into_string()
        .map_err(|_| "Failed to convert hostname to string")?;
    let home_dir = home::home_dir()
        .ok_or("Failed")?;

    Ok((username, host, home_dir))
}

fn print_prompt(username: &String, host: &String, home_dir: &PathBuf) {
    match env::current_dir() {
        Ok(path) => {
            let current_dir = path.strip_prefix(home_dir).expect("Failed");
            print!("{}@{}:~/{}{} ", username, host, current_dir.display(), PROMPT_CHAR);
        }
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
        }
    }
  
    
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("DEBUG: Raw input: {:?}", input);

    input.trim().to_string()
}

fn tokenize_command(c: String) -> (String, Vec<String>) {
    let mut parts = c.split_whitespace();
    let command = parts.next().unwrap().to_string();
    let args = parts.map(|s| s.to_string()).collect::<Vec<String>>();
    (command, args)
}

fn process_command(c: String, a: Vec<String> ) -> i32 {
    match Builtin::from_str(&c) {
        Ok(Builtin::Echo) => builtin_echo(&a),
        Ok(Builtin::History) => builtin_history(&a),
        Ok(Builtin::Cd) => builtin_cd(&a),
        Ok(Builtin::Pwd) => builtin_pwd(&a),
        _ => {
            let child = Command::new(&c)
                .args(&a)
                .spawn();

            match child {
                Ok(mut child) => { let _ = child.wait(); },
                Err(e) => eprintln!("{}", e),
            };
            1
        },
    }
}

fn builtin_echo(args : &Vec<String>) -> Result<(), String> {
    println!("{}", args.join(" "));
    Ok(())
}

fn builtin_history(args : &Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}

fn builtin_cd(args : &Vec<String>) -> Result<(), String> {
    let new_dir = if let Some(path) = args.get(0) {
        Path::new(path).to_path_buf()
    } else {
        home::home_dir().ok_or("No home directory found.")?
    };

    match env::set_current_dir(&new_dir) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err(format!("{} not found: No such file or directory.", new_dir.display()))
        },
    }
}

fn builtin_pwd(args : &Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}

// #[cfg(test)]
// mod unittest_tokenize_command {
//     use super::*;

//     #[test]
//     #[ignore]
//     fn empty_command() {
//       assert_eq!("", tokenize_command("".to_string()).keyword)
//     }

//     #[test]
//     fn test_keyword() {
//       assert_eq!("test", tokenize_command("test".to_string()).keyword)
//     }

//     #[test]
//     fn no_arg() {
//       assert_eq!(0, tokenize_command("test".to_string()).args.len())
//     }

//     #[test]
//     fn one_arg() {
//       assert_eq!(1, tokenize_command("test one".to_string()).args.len())
//     }

//     #[test]
//     fn multi_args() {
//       assert_eq!(3, tokenize_command("test one two three".to_string()).args.len())
//     }

//     #[test]
//     #[ignore]
//     fn quotes() {
//       assert_eq!(2, tokenize_command("test \”one two\” three".to_string()).args.len())
//     }
// }
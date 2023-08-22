use std::str::FromStr;

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


fn builtin_echo(args : &Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}

fn builtin_history(args : &Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}

fn builtin_cd(args : &Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}

fn builtin_pwd(args : &Vec<String>) -> i32 {
    println!("{}", args.join(" "));
    0
}
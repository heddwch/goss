use std::env::Args;
use std::collections::HashMap;
use std::iter::{ Peekable, Skip };

pub struct Opt {
    pub switch: char,
    pub argument: Option<String>,
}

pub struct GotOpt {
    pub opts: Vec<Opt>,
    pub rest: Peekable<Skip<Args>>,
}

pub enum Error {
    BadOptionString,
    UnknownSwitch(char),
    MissingArgument(char),
}

pub type Result<T> = std::result::Result<T, Error>;

enum ArgType {
    NoArg,
    RequiresArg,
    AcceptsArg,
}

pub fn getopt(args: Args, optstring: &str) -> Result<GotOpt> {
    if optstring.chars().count() == 0 { return Err(Error::BadOptionString); }
    let mut arg_types = HashMap::new();
    let mut characters = optstring.chars().peekable();
    loop {
        let current = {
            match characters.next() {
                Some(switch) => switch,
                None => break,
            }
        };
        if !current.is_alphabetic() { return Err(Error::BadOptionString) };
        let takes_args = {
            match characters.peek() {
                Some(next_char) => {
                    if *next_char == ':' {
                        true
                    } else {
                        false
                    }
                },
                None => false,
            }
        };
        if takes_args {
            let _ = characters.next();
            let arg_optional = match characters.peek() {
                Some(next_char) => {
                    if *next_char == ':' {
                        true
                    } else {
                        false
                    }
                },
                None => false,
            };
            if arg_optional {
                let _ = characters.next();
                arg_types.insert(current, ArgType::AcceptsArg);
            } else {
                arg_types.insert(current, ArgType::RequiresArg);
            }
        } else {
            arg_types.insert(current, ArgType::NoArg);
        }
    }
    
    let mut args = args.skip(1).peekable();
    let mut opts: Vec<Opt> = Vec::new();
    loop {
        let more_args = match args.peek() {
            Some(next_arg) => {
                if next_arg.starts_with("-") {
                    true
                } else {
                    false
                }
            },
            None => false,
        };
        if more_args {
            let arg = args.next().unwrap();
            if !arg.starts_with("-") { break; }
            for character in arg.chars().skip(1) {
                match arg_types.get(&character) {
                    Some(arg_type) => {
                        let this_opt = Opt {
                            switch: character,
                            argument: match *arg_type {
                                ArgType::AcceptsArg|ArgType::RequiresArg => {
                                    let is_arg =  {
                                        let next_arg = args.peek();
                                        match next_arg {
                                            Some(it) => {
                                                if it.starts_with("-") {
                                                    false
                                                } else {
                                                    true
                                                }
                                            },
                                            None => false,
                                        }
                                    };
                                    if is_arg {
                                        Some(args.next().unwrap())
                                    } else {
                                        None
                                    }
                                }
                                ArgType::NoArg => None,
                            }
                        };
                        match *arg_type {
                            ArgType::RequiresArg => {
                                if this_opt.argument == None {
                                    return Err(Error::MissingArgument(character));
                                }                                  
                            },
                            ArgType::AcceptsArg|ArgType::NoArg => (),
                        };
                        opts.push(this_opt);
                    },
                    None => {
                        return Err(Error::UnknownSwitch(character));
                    },
                }
            }
        } else {
            break;
        }
    }
    Ok(GotOpt {
        opts: opts,
        rest: args,
    })
}

#[test]
fn it_works() {
}

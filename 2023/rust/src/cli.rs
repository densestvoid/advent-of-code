use std::{fs, str::FromStr, io};

pub enum ArgType<T: Clone + FromStr> where <T as FromStr>::Err: std::fmt::Display {
    Positional,
    Optional(String, T),
}

pub enum Arg<T: Clone + FromStr> where <T as FromStr>::Err: std::fmt::Display {
    Unparsed(ArgType<T>),
    Parsed(T),
}

impl<T: Clone + FromStr> Arg<T> where <T as FromStr>::Err: std::fmt::Display {
    pub fn new_positional() -> Self { Arg::Unparsed(ArgType::Positional) }

    pub fn new_optional(flag: String, default: T) -> Self { Arg::Unparsed(ArgType::Optional(flag, default)) }
}

trait ArgTrait {
    fn parse(&mut self, args: &mut Vec<String>) -> Result<(), ParseError>;
}

pub struct ParseError {
    pub reason: String,
}

impl<T: Clone + FromStr> ArgTrait for Arg<T> where <T as FromStr>::Err: std::fmt::Display {
    fn parse(&mut self, args: &mut Vec<String>) -> Result<(), ParseError> {
        match self {
            Arg::Unparsed(arg_type) => match arg_type {
                ArgType::Positional => *self = Arg::Parsed(match args.remove(0).parse() {
                    Ok(t) => t,
                    Err(e) => return Err(ParseError{reason: format!("failed to parse positional argument: {}", e)}),
                }),
                ArgType::Optional(flag, def) => match args.iter().position(|t| *t == format!("-{}", flag)) {
                    Some(t) => {
                        *self = Arg::Parsed(match args.remove(t+1).parse() {
                            Ok(t) => t,
                            Err(e) => return Err(ParseError{reason: format!("failed to parse optional argument: {}", e)}),
                        });
                        args.remove(t);
                    }
                    None => *self = Arg::Parsed(def.clone()),
                }
            }
            Arg::Parsed(_) => (),
        };

        Ok(())
    }
}

pub struct FileContents{
    pub contents: String,
}

impl FromStr for FileContents {
    type Err = io::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match fs::read_to_string(value) {
            Ok(s) => Ok(FileContents{contents: s}),
            Err(e) => Err(e),
        }
    }
}

impl Clone for FileContents {
    fn clone(&self) -> Self {
        FileContents{contents: self.contents.clone()}
    }
}

pub struct Parser<'a> {
    positionals: Vec<Box<&'a mut dyn ArgTrait>>,
    optionals: Vec<Box<&'a mut dyn ArgTrait>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser{positionals: Vec::new(), optionals: Vec::new()}
    }

    pub fn add_argument <T: Clone + FromStr>(&mut self, arg: &'a mut Arg<T>) where <T as FromStr>::Err: std::fmt::Display {
        match arg {
            Arg::Unparsed(arg_type) => match arg_type {
                ArgType::Positional => self.positionals.push(Box::new(arg)),
                ArgType::Optional(_, _) => self.optionals.push(Box::new(arg)),
            }
            Arg::Parsed(_) => ()
        }
        
    }

    pub fn parse(self, mut args: Vec<String>) -> Result<(), ParseError> {
        // Remove the filename as the first argument
        args.remove(0);

        // Parse optionals first, leaving only positionals behind
        for arg in self.optionals {
            arg.parse(&mut args)?;
        }

        for arg in self.positionals {
            arg.parse(&mut args)?;
        }

        if args.len() > 0 {
            return Err(ParseError { reason: format!("unparsed arguments: {:?}", args) })
        }

        Ok(())
    }
}
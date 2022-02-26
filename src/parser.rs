use crate::errors::ParserError;

#[derive(Debug)]
enum OptTypes<'a> {
    Switch(&'a mut bool),
    Value(&'a mut String),
}

#[derive(Debug)]
struct Opt<'a> {
    short_name: Option<char>,
    long_name: Option<String>,
    opt_type: OptTypes<'a>,
}

/// Parser instance
#[derive(Debug)]
pub struct Parser<'a> {
    opts: Vec<Opt<'a>>
}

impl<'a> Parser<'a> {
    /// Creates new parser
    pub fn new() -> Self {
        Self {
            opts: Vec::new(),
        }
    }

    /// Adds new switch option
    pub fn add_switch_opt(&mut self, long_name: Option<&str>, 
                          short_name: Option<char>, 
                          data: &'a mut bool
    ) -> &mut Self {
        let long_name = if long_name.is_some() {
            Some(long_name.unwrap().to_string())
        } else {
            None
        };

        self.opts.push(Opt { 
            short_name,
            long_name,
            opt_type: OptTypes::Switch(data),
        });
        self
    }
    
    /// Adds new value option
    pub fn add_value_opt(&mut self, long_name: Option<&str>, 
                          short_name: Option<char>, 
                          data: &'a mut String
    ) -> &mut Self {
        let long_name = if long_name.is_some() {
            Some(long_name.unwrap().to_string())
        } else {
            None
        };

        self.opts.push(Opt { 
            short_name,
            long_name,
            opt_type: OptTypes::Value(data),
        });
        self
    }

    /// Parses arguments with defined options
    ///
    /// Returns the non-option arguments if successful, else returns a ParserError
    pub fn parse_with(&mut self, args: Vec<String>) -> Result<Vec<String>, ParserError> {
        let mut args_iter = args.into_iter();

        loop {
            match args_iter.next() {
                Some(a) => {
                    if a == "--" { 
                        // End parsing on '--'
                        break
                    } else if a.starts_with("--") {
                        // Long options
                        let a = a.chars().skip(2).collect::<String>();
                        let mut matched = false;

                        for i in &mut self.opts {
                            if i.long_name.is_some() && a == *i.long_name.as_ref().unwrap() {
                                matched = true;

                                match &mut i.opt_type {
                                    OptTypes::Switch(d) => **d = true,
                                    OptTypes::Value(d) => **d = args_iter.next().unwrap().to_string(),
                                }
                            }
                        }

                        if !matched {
                            return Err(ParserError::UnknownOption(a))
                        }
                    } else if a.starts_with("-") {
                        // Short options
                        let mut char_iter = a.chars();
                        char_iter.next().unwrap();
                        loop {
                            match char_iter.next() {
                                Some(c) => {
                                    for i in &mut self.opts {
                                        if Some(c) == i.short_name {
                                            match &mut i.opt_type {
                                                OptTypes::Switch(d) => **d = true,
                                                OptTypes::Value(d) => {
                                                    let s = char_iter.as_str().to_string();

                                                    **d = if s.len() == 0 {
                                                        args_iter.next().unwrap().to_string()
                                                    } else { s };
                                                },
                                            }
                                        }
                                    }
                                },
                                None => break,
                            }
                        }
                    } else { 
                        // End parsing on first non-option
                        break
                    }
                },

                None => break,
            }
        }

        Ok(args_iter.collect())
    }

    /// Parses with standard arguments 
    pub fn parse(&mut self) -> Result<Vec<String>, ParserError> {
        self.parse_with(std::env::args().skip(1).collect())
    }
}

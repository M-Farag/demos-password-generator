use std::error::Error;

use structopt::StructOpt;
use rand::seq::SliceRandom;

#[derive(Debug, StructOpt)]
#[structopt(name="Password generator",about="A strong password generator")]
struct Arguments {
    #[structopt(long="length", short="l", default_value="8")]
    length:usize,

    #[structopt(long="lowercase", short="w")]
    is_lowercase:bool,

    #[structopt(long="uppercase", short="u")]
    is_uppercase:bool,

    #[structopt(long="numbers", short="n")]
    is_numbers:bool,

    #[structopt(long="special", short="s")]
    is_special_symbols:bool
}

impl Arguments {
    fn get() -> Result<Self,Box<dyn Error>>
    {
        let args:Arguments = Arguments::from_args_safe()?;
        let length:usize = args.length;
        if length < 5 || length > 80 {
            Err("The length must be between 5 and 80")?;
        }
        Ok(Self { length, ..args })
    }
}

#[derive(Debug)]
pub struct Password {
    as_text:String,
    length:usize
}

impl Password {
    
    pub fn new() -> Result<Self,Box<dyn Error>>
    {
        let args = Arguments::get()?;

        let mut chars_pool:Vec<char> = Vec::new();

        if args.is_lowercase {
            chars_pool.extend('a'..='z');
        }

        if args.is_uppercase {
            chars_pool.extend('A'..='Z');
        }

        if args.is_numbers {
            chars_pool.extend('0'..='9');
        }

        if args.is_special_symbols {
            chars_pool.extend("!@#$%^&*()_+-=[]{}|;':,./<>?".chars());
        }

        if chars_pool.is_empty() {
            Err("You must select at least one character type")?;
        }

        let as_text:String = (0..args.length).map(
            |_| {
                * chars_pool.choose(&mut rand::thread_rng()).unwrap()
            }
        ).collect();

       Ok( Self { as_text, length: args.length })
    }
}
use rand::seq::SliceRandom;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name="Password generator",about="A strong password generator")]
struct Arguments {
    #[structopt(long="length", short="l", default_value="8")]
    length:usize
}

fn main() {
    // Todo
    // - [x] Adding the structopt crate
    // - [x] Use the struct opt in the main.rs
    // - [x] bind and config the arguments expected from the end user
    // - [x] test that everything works

    let args = Arguments::from_args();

    println!("length is: {}",args.length);

    let chars_pool:Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-=".chars().collect();

    let password:String = (0..args.length).map(
        |_| {
            * chars_pool.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();

    println!("Password is: {}",password);
}

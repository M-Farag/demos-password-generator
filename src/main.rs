use rand::seq::SliceRandom;

fn main() {
    // Todo
    // - [x] build a logic to generate a password
    // - [x] testing that everything works

    let length:usize = 12;
    let chars_pool:Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-=".chars().collect();

    let password:String = (0..length).map(
        |_| {
            * chars_pool.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();

    println!("Password is: {}",password);
}

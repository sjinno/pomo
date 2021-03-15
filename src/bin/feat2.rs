use envy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    mins: u64,
    title: String,
}

fn main() {
    match envy::from_env::<Config>() {
        Ok(config) => println!("{:#?}", config),
        Err(error) => eprintln!("{:#?}", error),
    }
}

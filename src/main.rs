#[macro_use]
extern crate log;

mod configure;
mod error;

fn main() {
    env_logger::init();
    println!("Hello, world!");
}

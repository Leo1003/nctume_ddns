#[macro_use]
extern crate log;

mod configure;
mod ddns;
mod error;

fn main() {
    env_logger::init();

    let conf = configure::ClientConf::load().unwrap_or_else(|e| {
        error!("{}", e);
        panic!();
    });

    if conf.interval_min() == 0 {
        error!("Update interval should larger than 0!");
        panic!();
    }


}

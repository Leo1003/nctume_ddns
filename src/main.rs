#[macro_use]
extern crate log;

use env_logger::Env;
use std::thread::sleep;
use std::time::Duration;

mod configure;
mod ddns;
mod error;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        env_logger::from_env(Env::default().default_filter_or("nctume_ddns=trace")).init();
    } else {
        env_logger::from_env(Env::default().default_filter_or("nctume_ddns=info")).init();
    }

    let conf = configure::ClientConf::load()
        .map_err(|e| {
            error!("{}", e);
        })
        .unwrap();

    if conf.interval_min() == 0 {
        error!("Update interval should larger than 0!");
        panic!();
    }

    let mut record = ddns::DDnsRecord::init(conf.record_id(), conf.token()).await
        .map_err(|e| {
            error!("Failed to initialize DNS record: {:?}", e);
        })
        .unwrap();

    let mut retries = 0;
    loop {
        let next_sync = match record.update().await {
            Ok(_) => {
                retries = 0;
                info!("DDNS update successful");
                conf.interval()
            }
            Err(e) => {
                retries += 1;
                let mut retry_interval = conf.interval() / 10;
                if retry_interval.as_secs() > 120 {
                    retry_interval = Duration::from_secs(120);
                }
                error!("DDNS update failed: {:?}", e);
                info!("Retry in {} seconds", retry_interval.as_secs());
                info!("Failed {} times", retries);
                retry_interval
            }
        };
        sleep(next_sync);
    }
}

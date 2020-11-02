use std::path::PathBuf;

use parking_lot::Mutex;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "energonsoftware")]
pub struct Options {
    #[structopt(short, long, default_value = "0.0.0.0")]
    pub host: String,

    #[structopt(short, long, default_value = "8000")]
    pub port: u16,

    #[structopt(skip)]
    address: Mutex<Option<String>>,

    #[structopt(long, default_value = ".")]
    prefix: PathBuf,
}

impl Options {
    pub fn address(&self) -> String {
        let mut address = self.address.lock();
        match &*address {
            Some(address) => address.clone(),
            None => {
                let addr = format!("{}:{}", self.host, self.port);
                *address = Some(addr.clone());
                addr
            }
        }
    }

    #[allow(dead_code)]
    pub fn conf_dir(&self) -> PathBuf {
        self.prefix.join("etc")
    }

    #[allow(dead_code)]
    pub fn share_dir(&self) -> PathBuf {
        self.prefix.join("share")
    }
}

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
}

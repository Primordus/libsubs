
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;


pub trait Download {
    fn download(&self, url: &str) -> Result<Vec<u8>, String>;
}

pub struct Downloader;

impl Downloader {
    pub fn new() -> Downloader {
        Downloader
    }
}

impl Download for Downloader {
    fn download(&self, url: &str) -> Result<Vec<u8>, String> {
        let client = Client::new();
        let response = client.get(url)
                             .header(Connection::close())
                             .send();
        match response {
            Ok(mut res) => {
                let mut result = Vec::new();
                res.read_to_end(&mut result).unwrap();
                Ok(result)
            }
            Err(reason) => Err(format!("Problem downloading URL: {}", reason))
        }
    }
}

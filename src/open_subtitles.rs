
use std::fs::File;
use std::io::Write;

use hyper::Client;
use hyper::header::Connection;

use fetchable::{Fetchable,SubtitleError};
use xml_parser;
use movie_hash;


fn download(url: &str) -> Result<String, String> {
    let mut client = Client::new();
    let response = client.get(&url)
                         .header(Connection::close())
                         .send();
    match response {
        Ok(res) => {
            let mut result = String::new();
            res.read_to_string(&mut result);
            Ok(result)
        }
        Err(reason) => Err(format!("Problem downloading URL: {}", reason))
    }   
}

// TODO fix ordering!
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Subtitle {
    url: String,
    rating: u8
}

impl Subtitle {
    fn new(url: String, rating: u8) -> Subtitle {
        Subtitle {
            url: url,
            rating: rating
        }
    }

    fn download(&self) -> Result<Vec<u8>, String> {
        Ok(vec![1]) // TODO
    }
}

pub struct OpenSubtitles {
    base_url: String,
    tmp_dir: String
}

// TODO move some functions to subtitle struct?
impl OpenSubtitles {
    pub fn new() -> OpenSubtitles {
        OpenSubtitles {
            base_url: "http://www.opensubtitles.org".to_string(),
            tmp_dir: "/tmp/".to_string()
        }
    }

    fn get_hash(&self, episode_name: &str) -> Result<u64, &'static str> {
        movie_hash::compute_hash(episode_name)
    }

    fn get_sub_list_xml(&self, movie_hash: u64, episode_name: &str, language: &str) 
                        -> Result<String, String> {
        let url = self.base_url.clone() + "/en/search/sublanguageid-" + language 
            + "/moviehash-" + stringify!(movie_hash) + "/simplexml";
        download(&url)
    }

    fn parse_sub_list_xml(&self, xml: String) -> Result<Vec<Subtitle>, String> {
        let sub_urls = xml_parser::parse(&xml, "//download/text()"); // TODO unwrap both
        let sub_ratings = xml_parser::parse(&xml, "//subrating/text()");

        if sub_urls.len() == 0 || sub_ratings.len() == 0 {
            return Err("No matching subtitles found!");
        }

        let subtitle_list = sub_urls
            .into_iter()
            .zip(sub_ratings.into_iter())
            .map(|(sub_url, sub_rating)| {
                Subtitle::new(sub_url, sub_rating)
            }).collect();

        Ok(subtitle_list)   
    }

    // TODO move function out, self is not used..
    fn choose_best_srt(&self, subs: &mut Vec<Subtitle>) -> Subtitle {
        subs.sort();
        subs.swap_remove(0)
    }

    fn download_subtitle(&self, sub: Subtitle, episode_name: &str) -> Result<String, String> {
        // TODO refactor the download function..
        let mut client = Client::new();
        let response = client.get(&sub.url)
                        .header(Connection::close())
                        .send();       
        match response {
            Ok(res) => {
                let episode = "".to_string().split("/").last().unwrap();  // TODO refactor to format function..
                let zip_location = self.tmp_dir.clone() + episode + ".zip";
                let mut file = try!(File::open("zip_location"));
                try!(file.write_all(res));
                Ok(zip_location)               
            }
            Err(reason) => Err(format!("Problem downloading subtitle: {}", reason))
        }
    }

    fn unzip_and_move(&self, zip_location: String) -> Result<String, String> {
        use std::process::Command;

        let episode = "";  // TODO format
        let unzip_dir = self.tmp_dir.clone() + episode + "/";
        let cmd = Command::new("unzip -o ".to_string() + &zip_location 
                                  + " -d " + &unzip_dir).status();
        match cmd {
            Err(reason) => return Err(format!("Problem unzipping to {}: {}", unzip_dir, reason)),
            _ => {}
        }

        /*
         * TODO
         * check if that dir now contains a .srt
         * rename file to episode_name but with extension .srt
         * print out some information if it has been download or print on error
         */

        Ok(format!("Downloaded subtitle for {}.", episode))
    }
}

impl Fetchable for OpenSubtitles {
    fn fetch(&self, episode_name: &str, language: &str) -> Result<String, SubtitleError> {
        let hash = try!(self.get_hash(episode_name)
                            .map_err(|_e| SubtitleError::HashError));
        let xml = try!(self.get_sub_list_xml(hash, episode_name, language)
                           .map_err(|e| SubtitleError::DownloadError(e)));
        let subtitles = try!(self.parse_sub_list_xml(&xml).map_err(|e| SubtitleError::XmlParseError(e)));
        let subtitle = self.choose_best_srt(subtitle_list);
        let zip_location = self.download_subtitle(subtitle, episode_name);
        self.unzip_and_move(zip_location)
    }
}

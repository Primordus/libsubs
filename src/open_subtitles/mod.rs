
// Module structure:

mod movie_hash;
mod subtitle;
pub mod error;

#[cfg(test)]
mod test;

// Actual implementation:

use std::cmp::Ordering;

use xml_parser;
use downloader::Download;
use fetchable::Fetchable;
use subtitle_error::SubtitleError;

use open_subtitles::subtitle::Subtitle;
use open_subtitles::error::Error as OpenSubtitleError;

pub struct OpenSubtitles {
    base_url: String,
    tmp_dir: String,
    downloader: Box<Download>
}

impl Fetchable for OpenSubtitles {
    fn fetch(&self, episode_name: &str, language: &str) -> Result<String, SubtitleError> {
        let result = try!(self.do_fetch(episode_name, language));
        Ok(result)
    }
}

impl OpenSubtitles {
    pub fn new(dl: Box<Download>) -> OpenSubtitles {
        OpenSubtitles {
            base_url: "http://www.opensubtitles.org".to_string(),
            tmp_dir: "/tmp/".to_string(),
            downloader: dl
        }
    }
    
    fn do_fetch(&self, episode_name: &str, language: &str) -> Result<String, OpenSubtitleError> {
        let hash = try!(self.get_hash(&episode_name));
        let xml = try!(self.get_sub_list_xml(hash, language)
                           .map_err(|e| OpenSubtitleError::DownloadError(e)));
        let mut subtitles = try!(self.parse_sub_list_xml(xml));
        let zip_location = try!(self.download_best_subtitle(&mut subtitles, &episode_name));
        self.unzip_and_move(zip_location, &episode_name)
    }

    fn get_hash(&self, episode_name: &str) -> Result<u64, movie_hash::HashError> {
        movie_hash::compute_hash(episode_name)
    }

    fn get_sub_list_xml(&self, movie_hash: u64, language: &str) -> Result<String, String> {
        let url = format!("{}/en/search/sublanguageid-{}/moviehash-{}/simplexml", 
                          self.base_url, language, movie_hash);
        self.downloader.download(&url)
    }

    fn parse_sub_list_xml(&self, xml: String) -> Result<Vec<Subtitle>, OpenSubtitleError> {
        let sub_urls = try!(xml_parser::parse(&xml, "//download/text()"));
        let sub_ratings = try!(xml_parser::parse(&xml, "//subrating/text()"));

        if sub_urls.len() == 0 || sub_ratings.len() == 0 {
            return Err(OpenSubtitleError::NoSubtitlesFound);
        }

        let subtitle_list = sub_urls
            .into_iter()
            .zip(sub_ratings.into_iter())
            .map(|(sub_url, sub_rating)| {
                let rating = match sub_rating.parse::<f64>() {
                    Ok(float) => float,
                    Err(_reason) => 0.0
                };
                Subtitle::new(sub_url, rating)
            }).collect();

        Ok(subtitle_list)   
    }

    fn download_best_subtitle(&self, subtitles: &mut Vec<Subtitle>, episode_name: &str) 
                              -> Result<String, OpenSubtitleError> {
        assert!(subtitles.len() > 0);
        subtitles.sort_by(compare_subtitles);
        subtitles[0].download(&self.downloader, &episode_name)
    }

    fn unzip_and_move(&self, zip_location: String, episode_name: &str) -> Result<String, OpenSubtitleError> {
        use std::process::Command;

        let episode = format_episode(episode_name);
        let unzip_dir = self.tmp_dir.clone() + &episode + "/";
        let cmd = Command::new("unzip -o ".to_string() + &zip_location 
                                  + " -d " + &unzip_dir).status();
        if cmd.is_err() {
            return Err(OpenSubtitleError::FileError("Problem unzipping to tmp dir!".to_string()));
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


// Helper functions:

fn format_episode(episode_name: &str) -> String {
    let episode = episode_name.to_string();
    let result = episode.clone();  // figure out how to remove this copy..
    
    match episode.split(".").last() {
        Some(ext) => {
            if file_name_consists_of_multiple_parts(&ext, &result) {
                return result.replace(ext, "srt");
            }
        }
        None => {}
    }

    result + ".srt"
}

fn file_name_consists_of_multiple_parts(a: &str, b: &str) -> bool {
    a != b
}

fn compare_subtitles(sub_a: &Subtitle, sub_b: &Subtitle) -> Ordering {
    // Returns Greater if sub_a < sub_b, Less if sub_a > sub_b
    sub_a.rating.partial_cmp(&sub_b.rating).unwrap().reverse()
}


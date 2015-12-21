
use super::*;
use super::format_episode;
use super::compare_subtitles;
use super::subtitle::Subtitle;
use downloader::Download;

use std::rc::Rc;
use std::env;
use std::cmp::Ordering;


struct MockDownloader;

impl MockDownloader {
    fn new() -> MockDownloader { MockDownloader }
}

impl Download for MockDownloader {
    fn download(&self, url: &str) -> Result<String, String> {
        Ok(String::new() + url)
    }
}

#[test]
fn test_compare_subtitles() {
    let rating1 = 3.5;
    let rating2 = 4.2;

    let a = Subtitle::new("bla".to_string(), rating1.clone());
    let b = Subtitle::new("bla".to_string(), rating2.clone());
    assert_eq!(compare_subtitles(&a, &b), Ordering::Greater);

    // Equality doesn't work with floats! we just pick 1 of the 2 since same rating anyway

    let c = Subtitle::new("bla".to_string(), rating2.clone());
    let d = Subtitle::new("bla".to_string(), rating1.clone());
    assert_eq!(compare_subtitles(&c, &d), Ordering::Less);
}

#[test]
fn test_format_episode() {
    let name = "/tmp/fake_episode1.x265.avi";
    let formatted_name = format_episode(&name);
    assert_eq!(&formatted_name, "/tmp/fake_episode1.x265.srt");
    
    let name2 = "/tmp/fake_episode2";
    let formatted_name2 = format_episode(&name2);
    assert_eq!(&formatted_name2, "/tmp/fake_episode2.srt");
}

#[test]
fn test_get_sub_list_xml() {
    let mock_dl = Box::new(MockDownloader::new());
    let mock_hash = 123456789;
    let mock_language = "eng";
    let os = OpenSubtitles::new(mock_dl);

    let result = os.get_sub_list_xml(mock_hash, mock_language).unwrap();
    assert_eq!(result, "http://www.opensubtitles.org/en/search/sublanguageid-eng/moviehash-123456789/simplexml");
}

#[test]
fn test_parse_sub_list_xml() {
    // TODO move to file
    let xml1 = r#"
    <search>
    <base>http://www.opensubtitles.org/en</base>
    <results items="1" itemsfound="371117" searchtime="0.01">
    <subtitle>
    <download>http://dl.opensubtitles.org/en/download/subad/123456789</download>
    <detail>
    /subtitles/123456789/episode_name_here
    </detail>
    <iso639>en</iso639>
    <user/>
    <releasename>
    <![CDATA[releasenameinfo here..]]>
    </releasename>
    <idsubtitle>123456789</idsubtitle>
    <subadddate>2015-12-19 03:21:39</subadddate>
    <subrating>3.5</subrating>
    <subcomments>
    <![CDATA[ 0 ]]>
    </subcomments>
    <movie>
    <![CDATA[ "more information about movie here" ]]>
    </movie>
    <files>1</files>
    <format>srt</format>
    <language>English</language>
    </subtitle>
    <subtitle>
    <download>http://dl.opensubtitles.org/en/download/subad/987654321</download>
    <detail>
    /subtitles/987654321/episode_name_here
    </detail>
    <iso639>en</iso639>
    <user/>
    <releasename>
    <![CDATA[release name info here..]]>
    </releasename>
    <idsubtitle>987654321</idsubtitle>
    <subadddate>2015-12-19 03:21:39</subadddate>
    <subrating>4.2</subrating>
    <subcomments>
    <![CDATA[ 0 ]]>
    </subcomments>
    <movie>
    <![CDATA[ "more information about movie here" ]]>
    </movie>
    <files>1</files>
    <format>srt</format>
    <language>English</language>
    </subtitle>
    </results>
    </search>"#;
    let mock_dl = Box::new(MockDownloader::new());
    let os = OpenSubtitles::new(mock_dl);
    let result1 = os.parse_sub_list_xml(xml1.to_string()).unwrap();
    let rating1_1 = &result1[0].rating;
    let rating1_2 = &result1[1].rating;
    assert_eq!(result1.len(), 2);
    assert_eq!(&result1[0].url, "http://dl.opensubtitles.org/en/download/subad/123456789");
    assert!(rating1_1 > &3.4 && rating1_1 < &3.6);
    assert_eq!(&result1[1].url, "http://dl.opensubtitles.org/en/download/subad/987654321");
    assert!(rating1_2 > &4.1 && rating1_2 < &4.3);

    /* TODO enable again after using other dependency!
    let xml2 = r#"
    <search>
    <base>http://www.opensubtitles.org/en</base>
    <results items="0" itemsfound="371117" searchtime="0.01">
    </results>
    </search>"#;
    let result2 = os.parse_sub_list_xml(xml2.to_string());
    assert!(result2.is_err());
    match result2 {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, SubtitleError::NoSubtitlesFound)
    }

    let xml3 = "insert invalid xml here".to_string();
    let result3 = os.parse_sub_list_xml(xml3.to_string());
    assert!(result3.is_err());
    match result3 {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, SubtitleError::XmlError)
    }*/
}

#[test]
fn test_download_best_subtitle() {
    let episode = "bla";
    let mock_dl = Box::new(MockDownloader::new());
    let os = OpenSubtitles::new(mock_dl);
    let best_url = "url3".to_string();
    let mut subs = vec![Subtitle::new("url1".to_string(), 4.5),
                        Subtitle::new("url2".to_string(), 3.0),
                        Subtitle::new(best_url.clone(), 5.0)];
    let result = os.download_best_subtitle(&mut subs, episode).unwrap();
    assert_eq!(result, env::temp_dir().into_os_string()
                                      .into_string()
                                      .unwrap()
                        + episode + ".srt.zip");
    let best_rating = &subs[0].rating;
    assert!(best_rating > &4.9 && best_rating < &5.1);
}

#[test]
fn test_unzip_and_move() {
    // TODO!
    
    /*
    fn unzip_and_move(&self, zip_location: String) -> Result<String, SubtitleError> {
        use std::process::Command;

        let episode = "";  // TODO format
        let unzip_dir = self.tmp_dir.clone() + episode + "/";
        let cmd = Command::new("unzip -o ".to_string() + &zip_location 
                                  + " -d " + &unzip_dir).status();
        if cmd.is_err() {
            return Err(SubtitleError::UnzipError);
        }

        /*
         * TODO
         * check if that dir now contains a .srt
         * rename file to episode_name but with extension .srt
         * print out some information if it has been download or print on error
         */

        Ok(format!("Downloaded subtitle for {}.", episode))
    }*/

}

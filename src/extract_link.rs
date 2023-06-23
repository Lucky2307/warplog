pub mod extract_link {
    use std::{collections::HashMap, io::ErrorKind, io::Error};
    use regex::{Regex};
    #[derive(Debug)]
    pub struct ParsedLink {
        url: String,
        queries: HashMap<String, String>,
    }
    pub fn from_string(data: &str) -> Result<ParsedLink, Error> {
        let link_regex = Regex::new(r"https:\/\/.+?getGachaLog").unwrap();
        let link = match link_regex.captures_iter(&data).into_iter().last() {
            None => return Err(Error::new(ErrorKind::NotFound, "Link not found")),
            Some(found_link) => found_link.get(0).unwrap().as_str().to_owned(),
        };

        let queries_regex_raw = vec![
            r"authkey_ver=[A-z0-9%]+",
            r"region=[A-z0-9%]+",
            r"lang=[A-z0-9%]+",
            r"authkey=[A-z0-9%]+",
            r"game_biz=[A-z0-9%]+",
        ];

        let mut queries_hashmap = HashMap::new();
        for query_regex_raw in queries_regex_raw {
            let query_regex = Regex::new(query_regex_raw).unwrap();
            let query = match query_regex.captures_iter(&data).into_iter().last(){
                None => return Err(Error::new(ErrorKind::NotFound, "Required query not found")),
                Some(found_query) => found_query.iter().last().unwrap().unwrap().as_str(),
            };
            let query_split: Vec<&str> = query.split("=").collect();
            queries_hashmap.insert(query_split[0].to_owned(), query_split[1].to_owned());
        };
        Ok(ParsedLink {
            url: link,
            queries: queries_hashmap,
        })
    }
}

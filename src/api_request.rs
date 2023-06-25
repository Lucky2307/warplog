pub mod api_requests {
    use serde_derive::Deserialize;
    use std::{io::Error, io::ErrorKind};

    #[derive(Deserialize, Debug)]
    pub struct ListData {
        uid: String,
        gacha_id: String,
        gacha_type: String,
        item_id: String,
        count: String,
        time: String,
        name: String,
        lang: String,
        item_type: String,
        rank_type: String,
        id: String,
    }
    #[derive(Deserialize, Debug)]
    struct ResponseData {
        page: String,
        size: String,
        list: Vec<ListData>,
        region: String,
        region_time_zone: u8,
    }
    #[derive(Deserialize, Debug)]
    struct WarpResponse {
        retcode: i8,
        message: String,
        data: ResponseData,
    }
    pub fn get_warp_data(base_link: String) -> Result<Vec<(u8, Vec<ListData>)>, Error> {
        let gacha_types: Vec<u8> = vec![
            1,  // Permanent
            2,  // Departure
            11, // Character
            12, // Light cone
        ];
        let mut result: Vec<(u8, Vec<ListData>)> = vec![];
        for gacha_type in gacha_types {
            let mut list_data: Vec<ListData> = vec![];
            let mut end_id: String = "0".to_owned();
            while {
                let mut res: WarpResponse = reqwest::blocking::get(format!(
                    "{base_link}&gacha_type={gacha_type}&end_id={end_id}"
                ))
                .unwrap()
                .json()
                .unwrap();
                match &res.retcode {
                    0 => {}
                    -100 => return Err(Error::new(ErrorKind::InvalidInput, "Invalid authkey")),
                    -101 => return Err(Error::new(ErrorKind::TimedOut, "Authkey timed out")),
                    _ => return Err(Error::new(ErrorKind::InvalidInput, "Generic error")),
                }
                end_id = res.data.list.last().unwrap().id.as_str().to_owned();
                let list_len = res.data.list.len();
                list_data.append(&mut res.data.list);
                list_len >= 20
            } {}
            result.append(&mut vec![(gacha_type, list_data)]);
        }
        Ok(result)
    }
}

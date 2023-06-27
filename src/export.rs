pub mod export {
    use crate::api_request::api_requests::Response;
    use std::error::Error;

    impl Response {
        pub fn to_csv(&self) -> Result<(), Box<dyn Error>> {
            let mut writer = csv::WriterBuilder::new()
                .delimiter(b'\t')
                .from_path(format!("out\\export.csv"))?;
            for item in self.get_data() {
                writer.write_record(item.get_presentable_data())?;
            }
            writer.flush()?;
            Ok(())
        }
    }
}

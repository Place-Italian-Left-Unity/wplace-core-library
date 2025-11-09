use crate::{GenericBytes, map_coords::MapCoords};

#[derive(serde::Deserialize)]
pub struct NominatimData {
    pub(crate) display_name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum NominatimDataError {
    #[error("JSON Deserialize Error: {error}; {input_value}")]
    JSONDeserializeError {
        error: serde_json::Error,
        input_value: String,
    },
}

static mut LAST_REQUEST: std::time::SystemTime = std::time::UNIX_EPOCH;

impl NominatimData {
    /// This function has side effects!
    ///
    /// It modifies a `LAST_REQUEST` global variable used to keep track of timeouts in the Nominatim API
    ///
    /// This *WILL* make the thread sleep!
    pub fn load_data(v: &MapCoords) -> Result<Self, NominatimDataError> {
        while let v = std::time::SystemTime::now().duration_since(unsafe { LAST_REQUEST })
            && (v.is_err() || v.unwrap().as_secs() < 1)
        {
            std::thread::sleep(std::time::Duration::new(1, 0));
        }
        unsafe { LAST_REQUEST = std::time::SystemTime::now() };

        let url = v.get_nominatim_link();

        let mut curl_client = curl::easy::Easy2::new(GenericBytes(Vec::with_capacity(1024)));
        curl_client.url(&url).expect("Couldn't select url");
        curl_client
            .useragent("Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0")
            .expect("Couldn't set user agent");
        curl_client.perform().expect("Couldn't perform");

        let data = curl_client.get_ref().0.clone();

        serde_json::from_slice(&data).map_err(|e| NominatimDataError::JSONDeserializeError {
            error: e,
            input_value: String::from_utf8_lossy(&data).to_string(),
        })
    }

    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
}

use geojson::PolygonType;

use crate::{GenericBytes, map_coords::MapCoords};

#[derive(Clone)]
pub struct NominatimData {
    pub(crate) display_name: String,
}

#[derive(serde::Deserialize)]
struct NominatimDataJson {
    display_name: String,
    addresstype: String,
    geojson: serde_json::Value,
}

#[derive(thiserror::Error, Debug)]
pub enum NominatimDataError {
    #[error("JSON Deserialize Error: {error}; {input_value}")]
    JSONDeserializeError {
        error: serde_json::Error,
        input_value: String,
    },
    #[error("GeoJSON Error: {0}")]
    GeoJSONError(#[from] geojson::Error),
    #[error("Invalid GeoJSON Type: {0}")]
    GeoJSONInvalidType(&'static str),
}

static mut LAST_REQUEST: std::time::SystemTime = std::time::UNIX_EPOCH;
static mut CACHE: Vec<(PolygonType, NominatimData, std::time::SystemTime)> = vec![];

impl NominatimData {
    /// This function has side effects!
    ///
    /// It modifies a `LAST_REQUEST` global variable used to keep track of timeouts in the Nominatim API
    ///
    /// This *WILL* make the thread sleep!
    pub fn load_data(v: &MapCoords) -> Result<Self, NominatimDataError> {
        unsafe {
            #[allow(static_mut_refs)]
            CACHE.retain(|(_, _, t)| {
                let v = std::time::SystemTime::now().duration_since(*t);
                v.is_ok() && v.unwrap().as_secs() < 1200
            });

            #[allow(static_mut_refs)]
            for (polygon, data, _) in &CACHE {
                let Some(outer) = polygon.first() else {
                    continue;
                };
                let mut outer = outer.iter().peekable();

                let mut count = 0;

                let lat = v.get_lat();
                let lng = v.get_lng();

                while let (Some(point), Some(next_point)) = (outer.next(), outer.peek()) {
                    let mut points = [
                        (*point.first().unwrap(), *point.get(1).unwrap()),
                        (*next_point.first().unwrap(), *next_point.get(1).unwrap()),
                    ];
                    points.sort_by(|a, b| a.1.total_cmp(&b.1));
                    let [below_point /* A */, above_point /* B */] = points;

                    if lat < below_point.1
                        || lat > above_point.1
                        || lng > f64::max(above_point.0, below_point.0)
                    {
                        continue;
                    }

                    let m_red = if (below_point.0 - above_point.0).abs() > 0.0 {
                        (above_point.1 - below_point.1) / (above_point.0 - below_point.0)
                    } else {
                        f64::MAX
                    };
                    let m_blue = if (below_point.0 - lng).abs() > 0.0 {
                        (lat - below_point.1) / (lng - below_point.0)
                    } else {
                        f64::MAX
                    };
                    if m_blue >= m_red {
                        count += 1
                    }
                }

                if count % 2 == 1 {
                    return Ok(data.clone());
                }
            }
        }

        while let v = std::time::SystemTime::now().duration_since(unsafe { LAST_REQUEST })
            && (v.is_err() || v.unwrap().as_secs() < 1)
        {
            std::thread::sleep(std::time::Duration::new(1, 100));
        }
        unsafe { LAST_REQUEST = std::time::SystemTime::now() };

        let url = v.get_nominatim_link();
        println!("{url}");

        let mut curl_client = curl::easy::Easy2::new(GenericBytes(Vec::with_capacity(1024)));
        curl_client.url(&url).expect("Couldn't select url");
        curl_client
            .useragent("Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0")
            .expect("Couldn't set user agent");
        curl_client.perform().expect("Couldn't perform");

        let data = curl_client.get_ref().0.clone();

        let data: NominatimDataJson = serde_json::from_slice(&data).map_err(|e| {
            NominatimDataError::JSONDeserializeError {
                error: e,
                input_value: String::from_utf8_lossy(&data).to_string(),
            }
        })?;

        let out = Self {
            display_name: data.display_name,
        };

        if data.addresstype == "country" {
            return Ok(out);
        }

        let geojson_value = geojson::Value::from_json_value(data.geojson)?;

        // Vec<Polygon>
        // Polygon: Vec<Exterior LinearRing, Interior LinearRing...>
        // LinearRing: Vec<Point>
        let polygons = match geojson_value {
            geojson::Value::Polygon(v) => vec![v; 1],
            geojson::Value::MultiPolygon(v) => v,
            v => return Err(NominatimDataError::GeoJSONInvalidType(v.type_name())),
        };

        for polygon in polygons {
            unsafe {
                #[allow(static_mut_refs)]
                CACHE.push((polygon.clone(), out.clone(), std::time::SystemTime::now()));
            }
        }

        Ok(out)
    }

    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
}

extern crate serde_json;
use url::Url;
use structs::*;
use cabot::{RequestBuilder, Client};

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleResponseResultGeometry {
    location: Location,
    location_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoogleResponseResult {
    geometry: GoogleResponseResultGeometry,
    formatted_address: String,
}

#[derive(Deserialize, Debug)]
pub struct GoogleResponse {
    pub results: Vec<GoogleResponseResult>,
    pub status: String,
}

pub const BASE_URL: &str = "https://maps.googleapis.com/maps/api/geocode/json";
pub const STATUS_NO_RESULTS: &str = "ZERO_RESULTS";
pub const STATUS_OK: &str = "OK";
pub const STATUS_OVER_LIMIT: &str = "OVER_QUERY_LIMIT";
pub const ERR_OVER_LIMIT: &str = "Exceeded public query limit.";
pub const ERR_NO_RESULTS: &str = "Could not find location by name.";

pub fn geocode(address: &str) -> Result<GeocodeResponse, String> {
    let mut url = Url::parse(BASE_URL).unwrap();
    url.query_pairs_mut().append_pair("address", address);
    let request = RequestBuilder::new(url.as_str())
        .set_http_method("GET")
        .build()
        .unwrap();
    let client = Client::new();
    let raw_response = client.execute(&request).unwrap();
    let response: GoogleResponse = serde_json::from_slice(
	    	raw_response.body().unwrap()
	    )
	    .unwrap();

    return match response.status.as_str() {
        STATUS_OK => Ok(GeocodeResponse {
            location: response.results[0].clone().geometry.location,
            location_type: response.results[0].clone().geometry.location_type,
            formatted_address: response.results[0].clone().formatted_address,
        }),
        STATUS_NO_RESULTS => Err(ERR_NO_RESULTS.to_string()),
        STATUS_OVER_LIMIT => Err(ERR_OVER_LIMIT.to_string()),
        _ => Err(response.status),
    };
}

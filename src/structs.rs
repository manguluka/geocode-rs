#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug)]
pub struct GeocodeResponse {
    pub location: Location,
    pub location_type: String,
    pub formatted_address: String,
}

extern crate cabot;
extern crate url;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod structs;
pub mod google;


#[cfg(test)]
mod tests {
    use google;
    #[test]
    fn returns_data_for_existing_place() {
        let result = google::geocode("santa rosa, ca").unwrap();
        println!("{:?}", result);
        assert_eq!(result.location.lat, 38.440429);
        assert_eq!(result.location.lng, -122.7140548);
    }

    #[test]
    fn returns_err_for_non_existing_place() {
        let result = google::geocode("santa dfdfdfdfdfd");
        println!("{:?}", result);
        assert_eq!(result.err(), Some(google::ERR_NO_RESULTS.to_string()));

    }
}

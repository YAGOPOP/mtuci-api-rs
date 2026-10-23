use crate::responses::schedule::types::ScheduleData;
use crate::types::ApiResponse;

#[test]
fn good_response() {
    let json = include_str!("good_response.json");
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let response: ApiResponse<ScheduleData> =
        serde_path_to_error::deserialize(&mut deserializer).unwrap();

    dbg!(response);
}

#[test]
fn bad_response() {
    let json = include_str!("bad_response.json");
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let response: ApiResponse<ScheduleData> =
        serde_path_to_error::deserialize(&mut deserializer).unwrap();

    dbg!(response);
}

#[test]
fn empty_good_response() {
    let json = include_str!("empty_good_response.json");
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let response: ApiResponse<ScheduleData> =
        serde_path_to_error::deserialize(&mut deserializer).unwrap();

    dbg!(response);
}

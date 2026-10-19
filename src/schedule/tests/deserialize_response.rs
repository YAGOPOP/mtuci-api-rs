use crate::schedule::types::ScheduleResponse;

#[test]
fn good_response() {
    let json = include_str!("good_response.json");
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let response: ScheduleResponse = serde_path_to_error::deserialize(&mut deserializer).unwrap();

    dbg!(response);
}

#[test]
fn bad_response() {
    let json = include_str!("bad_response.json");
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let response: ScheduleResponse = serde_path_to_error::deserialize(&mut deserializer).unwrap();

    dbg!(response);
}

#[test]
fn empty_good_response() {
    let json = include_str!("empty_good_response.json");
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let response: ScheduleResponse = serde_path_to_error::deserialize(&mut deserializer).unwrap();

    dbg!(response);
}

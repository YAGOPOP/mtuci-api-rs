use serde_json;

use super::super::types::ScheduleResponse;

#[test]
fn deserialize_good_response() {
    let good_response = include_str!("good_response.json");

    let response: ScheduleResponse = serde_json::from_str(good_response).unwrap();

    dbg!(response);
}

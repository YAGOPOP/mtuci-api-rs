use crate::responses::info::types::RoomsInfo;

#[test]
fn deserialize_rooms_info() {
    let json = include_str!("info.json");
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let response: RoomsInfo = serde_path_to_error::deserialize(&mut deserializer).unwrap();

    dbg!(response);
}

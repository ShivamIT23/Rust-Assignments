use serde::{Deserialize,Serialize};
use serde_json::Deserializer;

#[derive(Deserialize,Serialize,Debug)]
#[serde(rename_all="camelCase")]

struct SignupResponse{
    message : String,
    status_code : i32,
}

fn main() {
    let response = SignupResponse {
        message : String::from("You are not able to signup"),
        status_code : 200
    };

    let json_resp = serde_json::to_string(&response).unwrap();
    println!("{}",json_resp);
    let des_resp : SignupResponse = serde_json::from_str::<SignupResponse>(&json_resp).unwrap();
    println!("{:?}",des_resp);
}

//36:35

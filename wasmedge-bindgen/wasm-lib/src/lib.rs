use num_integer::lcm;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256, Sha3_256};
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
    points: Vec<Point>,
    valid: bool,
    length: f32,
    desc: String,
}

#[wasmedge_bindgen]
pub fn create_line(p1: String, p2: String, desc: String) -> String {
    let point1: Point = serde_json::from_str(p1.as_str()).unwrap();
    let point2: Point = serde_json::from_str(p2.as_str()).unwrap();
    let length = ((point1.x - point2.x) * (point1.x - point2.x)
        + (point1.y - point2.y) * (point1.y - point2.y))
        .sqrt();

    let valid = if length == 0.0 { false } else { true };

    let line = Line {
        points: vec![point1, point2],
        valid: valid,
        length: length,
        desc: desc,
    };

    return serde_json::to_string(&line).unwrap();
}

#[wasmedge_bindgen]
pub fn say_ok(s: String) -> Result<(u16, String), String> {
    let r = String::from("hello ");
    return Ok((5 as u16, r + s.as_str()));
}

#[wasmedge_bindgen]
pub fn say_err(s: String) -> Result<(u16, String), String> {
    let r = String::from("hello ");
    return Err(String::from("abc"));
}

#[wasmedge_bindgen]
pub fn obfusticate(s: String) -> String {
    (&s).chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}

#[wasmedge_bindgen]
pub fn lowest_common_multiple(a: i32, b: i32) -> i32 {
    return lcm(a, b);
}

#[wasmedge_bindgen]
pub fn sha3_digest(v: Vec<u8>) -> Vec<u8> {
    return Sha3_256::digest(&v).as_slice().to_vec();
}

#[wasmedge_bindgen]
pub fn keccak_digest(s: Vec<u8>) -> Vec<u8> {
    return Keccak256::digest(&s).as_slice().to_vec();
}

#![allow(unused)]

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
pub struct Document {
    version: String,
    entries: Vec<Entry>,
}

// we can separate tracks from detection.
// tracks have unique id + uas id
// add alarm certainty
// for the company: schema + documentation

#[derive(JsonSchema)]
pub struct Entry {
    #[validate(max_length = 5)]
    system_id: u32,
    #[validate(max_length = 3)]
    vendor_id: u32,
    time: String,
    record_number: u64,
    uas_ident: Option<String>,
    classification: Classification,
    alarm: bool,
    detection_id: Option<u64>,
    track_id: Option<u64>,
    location: Location,
    identification: String,
    cuas_origin: Position3d,
}

#[derive(JsonSchema)]
pub enum Classification {
    #[serde(rename = "UAV")]
    Uav,
    #[serde(rename = "GCS")]
    Gcs,
}

#[derive(JsonSchema)]
pub enum Location {
    // clockwise: from -> to (degrees)
    Arc(Arc),
    Quad(Quad),
    // clockwise from true north (degrees)
    Bearing(f64),
    Position(Position),
    Position3d(Position3d),
    // Bearing + elevation? (HGH)
    // Bearing + elevation + distance? (ART)
}

#[derive(JsonSchema)]
pub struct Arc {
    from: f64,
    to: f64,
}

#[derive(JsonSchema)]
pub struct Position {
    lat: f64,
    lon: f64,
}

#[derive(JsonSchema)]
pub struct Position3d {
    lat: f64,
    lon: f64,
    height: f64,
}

#[derive(JsonSchema)]
pub enum Quad {
    North,
    East,
    South,
    West,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(Document);
    let mut string = serde_json::to_string_pretty(&schema)?;
    BufWriter::new(File::create("courageous.schema.json")?).write_all(string.as_bytes())?;

    Ok(())
}

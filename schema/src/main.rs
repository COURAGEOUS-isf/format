#![allow(unused)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
};

use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
pub struct Document {
    // schema present too, which is where the version/standard is defined
    /// Detection sets present in the document, mapped by an unique ID.
    detection: HashMap<u64, Detection>,
    /// Tracks present in the document, mapped by an unique ID.
    tracks: HashMap<String, Track>,
}

#[derive(JsonSchema)]
pub struct Detection {
    /// An unique ID used to associate this detection with a specific UAS.
    /// The number itself is not relevant, it just needs to be unique per UAS.
    ///
    /// If null, means the system was not able to identify the records present along with this
    /// object with any specific UAS.
    uas_id: Option<u64>,
    /// A list of records associated with this detection.
    records: Vec<Record>,
}

#[derive(JsonSchema)]
pub struct Track {
    /// An unique ID used to associate this track with a specific UAS.
    /// The number itself is not relevant, it just needs to be unique per UAS.
    ///
    /// If null, means the system was not able to identify the records present along with this
    /// object with any specific UAS.
    uas_id: u64,
    /// A list of records associated with this track.
    records: Vec<Record>,
}

// for the company: schema + documentation
// System id and vendor id aren't relevant here because they are specific to CPNI (UK National Protective Security Authority) and they refer to the radar system
// TODO better docs

#[derive(JsonSchema)]
pub struct Record {
    /// UTC time in the RFC 3339 format for date and time (As described in https://datatracker.ietf.org/doc/html/rfc3339#section-5.6)
    #[schemars(regex(pattern = r"^((?:(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2}(?:\.\d+)?))Z?)$"))]
    time: String,
    /// An unique number that identifies this record between all other ones present in the document.
    record_number: u64,
    /// Classification of the record.
    classification: Classification,
    /// Whether the alarm function of the CUAS system is active or not.
    alarm: bool,
    /// How certainly should the alarm be on, as a value from 0 (Least likely) to 1 (Most likely).
    #[validate(range(min = 0., max = 1.))]
    alarm_certainty: f64,
    /// The UAS location, which may be given in one of several declaration types.
    location: Location,
    /// Free form text, possibly describing the model or configuration of the UAS identified.
    identification: String,
    /// The 3D GPS location of the CUAS recorded on this instant.
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
    BearingElevation {
        bearing: f64,
        elevation: f64,
    },
    BearingElevationDistance {
        bearing: f64,
        elevation: f64,
        distance: f64,
    },
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
    let mut string = serde_json::to_string(&schema)?;
    BufWriter::new(File::create("courageous.schema.json")?).write_all(string.as_bytes())?;

    Ok(())
}

#![allow(unused)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
};

use schemars::{schema_for, JsonSchema};

#[derive(JsonSchema)]
pub struct Document {
    /// The 3D GPS location of the CUAS recorded.
    // TODO change docs
    static_cuas_location: Position3d,
    // schema present too, which is where the version/standard is defined
    /// Detection sets present in the document, mapped by an unique ID.
    detection: HashMap<u64, Detection>,
    /// Tracks present in the document, mapped by an unique ID.
    tracks: HashMap<String, Track>,
    vendor_name: String,
    system_name: String,
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
    /// Free-form text.
    name: Option<String>,
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
    /// Free-form text.
    name: Option<String>,
}

// for the company: schema + documentation
// TODO better docs
// TODO fix required *
// TODO change detection & tracking to arrays

#[derive(JsonSchema)]
pub struct Record {
    /// UTC time in the RFC 3339 format for date and time (As described in
    /// https://datatracker.ietf.org/doc/html/rfc3339#section-5.6)
    #[schemars(regex(pattern = r"^((?:(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}:\d{2}(?:\.\d+)?))Z?)$"))]
    time: String,
    /// A unique number that identifies this record between all other ones present in the document.
    record_number: u64,
    /// Classification of the record.
    classification: Classification,
    /// Whether the alarm function of the CUAS system is active or not.
    /// An Alarm is defined as the function of a CUAS system alerting an Operator via the HMI and
    /// the generation of associated data in the UAS Activity Log, as a result of Declared UAS
    /// activity.
    alarm: bool,
    /// How certainly should the alarm be on, as a value from 0 (Least likely) to 1 (Most likely).
    #[validate(range(min = 0., max = 1.))]
    alarm_certainty: f64,
    /// The location of the target, which may be given in one of several declaration types.
    location: Location,
    /// Free form text, possibly describing the model or configuration of the UAS identified.
    identification: String,
    /// The 3D GPS location of the CUAS recorded on this instant. Overrides the document's
    /// static_cuas_location if included.
    cuas_location: Option<Position3d>,
}

#[derive(JsonSchema)]
pub enum Classification {
    #[serde(rename = "UAV")]
    Uav,
    #[serde(rename = "GCS")]
    Gcs,
}

#[derive(JsonSchema)]
/// Location of an object, which may be relative to the CUAS.
pub enum Location {
    // clockwise: from -> to (degrees)
    /// Circular arc relative to the CUAS within which the object resides.
    Arc(Arc),
    /// Compass quadrant where the UAS has been observed.
    Quad(Quad),
    // clockwise from true north (degrees)
    /// Clockwise angle in degrees from true north where the UAS has been observed.
    Bearing(f64),
    /// Flat 2D position given in Latitude and Longitude.
    Position2d(Position2d),
    /// 3D position given in Latitude, Longitude and height.
    Position3d(Position3d),
    /// Ray where the UAS has been observed, given in bearing and elevation
    BearingElevation {
        /// Clockwise angle in degrees from true north where the UAS has been observed.
        bearing: f64,
        /// Elevation over the horizon of the UAS relative to the CUAS given in degrees.
        elevation: f64,
    },
    /// 3D position of the UAS given in bearing, elevation angle and distance.
    BearingElevationDistance {
        /// Clockwise angle in degrees from true north where the UAS has been observed.
        bearing: f64,
        /// Elevation over the horizon of the UAS relative to the CUAS given in degrees.
        elevation: f64,
        /// Distance from the UAS to the CUAS given in meters.
        distance: f64,
    },
}

#[derive(JsonSchema)]
/// Circular arc within which the UAS has been observed.   
pub struct Arc {
    /// Minimum compass angle from the CUAS System to the UAS in degrees.
    from: f64,
    /// Maximum compass angle from the CUAS System to the UAS in degrees.
    to: f64,
}

#[derive(JsonSchema)]
/// Compass quadrant where the UAS has been observed.
pub enum Quad {
    North,
    East,
    South,
    West,
}

#[derive(JsonSchema)]
/// Flat 2D position of the UAS given in Latitude and Longitude.
pub struct Position2d {
    /// GPS latitude of the UAS measured in degrees.
    lat: f64,
    /// GPS longitude of the UAS measured in degrees.
    lon: f64,
}

#[derive(JsonSchema)]
/// 3D position of the UAS given in Latitude, Longitude and height.
pub struct Position3d {
    /// GPS latitude of the UAS measured in degrees.
    lat: f64,
    /// GPS longitude of the UAS measured in degrees.
    lon: f64,
    /// Height of the UAS measured in meters from sea level.
    height: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = schema_for!(Document);
    let mut string = serde_json::to_string(&schema)?;
    BufWriter::new(File::create("courageous.schema.json")?).write_all(string.as_bytes())?;

    Ok(())
}

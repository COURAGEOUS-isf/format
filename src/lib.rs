use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Document {
    /// The 3D GPS location of the CUAS. Can be overriden per Record, but even if overriden this
    /// value must exist and be a valid position.
    pub static_cuas_location: Position3d,
    // schema present too, which is where the version/standard is defined
    /// A list containing the detection sets present in the document.
    pub detection: Vec<Detection>,
    /// A list containing the tracks present in the document.
    pub tracks: Vec<Track>,
    pub vendor_name: String,
    pub system_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Detection {
    /// An unique ID used to associate this detection with a specific UAS.
    /// The number itself is not relevant, it just needs to be unique per UAS.
    ///
    /// If null, means the system was not able to identify the records present along with this
    /// object with any specific UAS.
    pub uas_id: Option<u64>,
    /// A list of records associated with this detection.
    pub records: Vec<Record>,
    /// Free-form text describing the detection set. Can be, for instance, the name present on the HMI.
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Track {
    /// An unique ID used to associate this track with a specific UAS.
    /// The number itself is not relevant, it just needs to be unique per UAS.
    pub uas_id: u64,
    /// A list of records associated with this track.
    pub records: Vec<Record>,
    /// Free-form text describing the track. Can be, for instance, the name present on the HMI.
    pub name: Option<String>,
}

// for the company: schema + documentation
// TODO better docs
// TODO fix required *
// TODO change detection & tracking to arrays

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Record {
    /// UTC time as an Unix millisecond timestamp.
    pub time: u64,
    /// A unique number that identifies this record between all other ones present in the document.
    pub record_number: u64,
    /// Classification of the record.
    pub classification: Classification,
    /// Whether the alarm function of the CUAS system is active or not.
    /// An Alarm is defined as the function of a CUAS system alerting an Operator via the HMI and
    /// the generation of associated data in the UAS Activity Log, as a result of Declared UAS
    /// activity.
    pub alarm: bool,
    /// How certainly should the alarm be on, as a value from 0 (Least likely) to 1 (Most likely).
    #[cfg_attr(feature = "schemars", validate(range(min = 0., max = 1.)))]
    pub alarm_certainty: f64,
    /// The UAS location, which may be given in one of several declaration types.
    pub location: Location,
    /// Free form text, possibly describing the model or configuration of the UAS identified.
    pub identification: Option<String>,
    /// The 3D GPS location of the CUAS recorded on this instant. Overrides the document's
    /// static_cuas_location.
    pub cuas_location: Option<Position3d>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum Classification {
    Unknown,
    #[serde(rename = "UAV")]
    Uav,
    #[serde(rename = "GCS")]
    Gcs,
    Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(tag = "t", content = "c")]
/// Location of an UAS, which may be relative to the CUAS.
pub enum Location {
    // clockwise: from -> to (degrees)
    /// Circular arc relative to the CUAS within which the UAS resides.
    Arc(Arc),
    /// Compass quadrant where the UAS has been observed.
    Quad(Quad),
    // clockwise from true north (degrees)
    /// Clockwise angle in degrees from true north where the UAS has been observed.
    Bearing(f64),
    /// Flat 2D position given in latitude and longitude.
    Position2d(Position2d),
    /// 3D position given in latitude, longitude and height.
    Position3d(Position3d),
    /// Ray where the UAS has been observed given in bearing and elevation.
    BearingElevation {
        /// Clockwise angle in degrees from true north where the UAS has been observed.
        bearing: f64,
        /// Elevation angle in degrees over the horizon where the UAS has been observed.
        elevation: f64,
    },
    /// 3D position of the UAS given in bearing, elevation angle and distance.
    BearingElevationDistance {
        /// Clockwise angle in degrees from true north where the UAS has been observed.
        bearing: f64,
        /// Elevation angle in degrees over the horizon where the UAS has been observed.
        elevation: f64,
        /// Distance from the UAS to the CUAS given in meters.
        distance: f64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
/// Describes a circular arc between two clockwise angles from true north.
pub struct Arc {
    /// Minimum compass angle from the CUAS System to the UAS in degrees.
    pub from: f64,
    /// Maximum compass angle from the CUAS System to the UAS in degrees.
    pub to: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
/// Describes a compass quadrant.
pub enum Quad {
    North,
    East,
    South,
    West,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
/// 2D WGS84 position given in latitude and longitude.
pub struct Position2d {
    /// GPS WGS84 latitude measured in degrees.
    pub lat: f64,
    /// GPS WGS84 longitude measured in degrees.
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
/// 3D WGS84 position given in latitude, longitude and height.
pub struct Position3d {
    /// GPS WGS84 latitude measured in degrees.
    pub lat: f64,
    /// GPS WGS84 longitude measured in degrees.
    pub lon: f64,
    /// Height measured in meters from sea level.
    pub height: f64,
}

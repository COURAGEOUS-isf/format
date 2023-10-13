use schemars::schema::SchemaObject;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Version(pub String);

impl Version {
    pub fn current() -> Self {
        let pkg_version = env!("CARGO_PKG_VERSION");
        let schema_version = pkg_version
            .split_once("+schema.")
            .map_or(pkg_version, |(_crate_version, schema_version)| {
                schema_version
            });
        Version(schema_version.to_owned())
    }
}

#[cfg(feature = "schemars")]
impl JsonSchema for Version {
    fn schema_name() -> String {
        "Version".to_owned()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        SchemaObject {
            enum_values: Some(vec![schemars::_serde_json::Value::String(
                Version::current().0,
            )]),
            ..Default::default()
        }
        .into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Document {
    /// The 3D GPS location of the CUAS. Can be overriden per Record, but even if overriden this
    /// value must exist and be a valid position.
    pub static_cuas_location: Position3d,
    // schema present too, which is where the version/standard is defined
    /// A list containing the detection sets present in the document.
    ///
    /// Detection sets should be used when the CUAS is surveying the perimeter for targets to detect.
    /// Their records are treated as independent entities.
    pub detection: Vec<Detection>,
    /// A list containing the tracks present in the document.
    ///
    /// Tracks should be used when the CUAS has locked on a target and is actively tracking its position.
    /// Their records describe the trajectory of a specific target.
    pub tracks: Vec<Track>,
    pub vendor_name: String,
    pub system_name: String,
    pub version: Version,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Detection {
    /// A unique ID used to associate this detection with a specific UAS.
    /// The number itself is not relevant, it just needs to be unique per UAS.
    ///
    /// If null, means the system was not able to identify the records present along with this
    /// object with any specific UAS.
    pub uas_id: Option<u64>,
    /// If this detection set refers to a specific UAV (i.e. the uas_id member is set) and if applicable, this member
    /// refers to the "home location" set on the UAV settings. This information can be available for C-UAS systems that
    /// listen to the data link protocol between the UAV and its GCS.
    pub uav_home_location: Option<Position3d>,
    /// A list of records associated with this detection.
    pub records: Vec<DetectionRecord>,
    /// Free-form text describing the detection set. Can be, for instance, the name present on the HMI.
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Track {
    /// A unique ID used to associate this track with a specific UAS.
    /// The number itself is not relevant, it just needs to be unique per UAS.
    pub uas_id: u64,
    /// If this track refers to a UAV and if applicable, this member refers to the "home location" set on the
    /// UAV settings. This information can be available for C-UAS systems that listen to the data link protocol between
    /// the UAV and its GCS.
    pub uav_home_location: Option<Position3d>,
    /// A list of records associated with this track.
    pub records: Vec<TrackingRecord>,
    /// Free-form text describing the track. Can be, for instance, the name present on the HMI.
    pub name: Option<String>,
}

/// A detection data record. Its main difference with tracking data records is the optionality of the `alarm` and
/// `location` members.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct DetectionRecord {
    /// UTC time as an Unix millisecond timestamp.
    pub time: u64,
    /// A unique number that identifies this record between all other ones present in the document.
    pub record_number: u64,
    /// Classification of the record.
    pub classification: Classification,
    /// If the record has Alarm data on this record, it may be specified here.
    pub alarm: Option<Alarm>,
    /// The UAS location, which may be given in one of several declaration types.
    ///
    /// On detection sets, this member may also be excluded if the position of the UAS is not known.
    pub location: Option<Location>,
    /// The UAS velocity given in ENU coordinates (given from the UAS position and given in meters per second).
    #[serde(default)]
    pub velocity: Option<CoordENU>,
    /// Free form text, possibly describing the model or configuration of the UAS identified.
    pub identification: Option<String>,
    /// The 3D GPS location of the CUAS recorded on this instant. Overrides the document's
    /// static_cuas_location.
    ///
    /// If the CUAS is located at the static_cuas_location on this instant, you may skip this field.
    pub cuas_location: Option<Position3d>,
}

/// A tracking data record. Its main difference with detection data records is the requirement of the `alarm` and
/// `location` members.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct TrackingRecord {
    /// UTC time as an Unix millisecond timestamp.
    pub time: u64,
    /// A unique number that identifies this record between all other ones present in the document.
    pub record_number: u64,
    /// Classification of the record.
    pub classification: Classification,
    /// Alarm data associated with this record.
    pub alarm: Alarm,
    /// The UAS location, which may be given in one of several declaration types.
    pub location: Location,
    /// The UAS velocity given in ENU coordinates (given from the UAS position and given in meters per second).
    #[serde(default)]
    pub velocity: Option<CoordENU>,
    /// Free form text, possibly describing the model or configuration of the UAS identified.
    pub identification: Option<String>,
    /// The 3D GPS location of the CUAS recorded on this instant. Overrides the document's
    /// static_cuas_location.
    ///
    /// If the CUAS is located at the static_cuas_location on this instant, you may skip this field.
    pub cuas_location: Option<Position3d>,
}

/// An Alarm is defined as the function of a CUAS system alerting an Operator via the HMI and
/// the generation of associated data in the UAS Activity Log, as a result of Declared UAS
/// activity.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Alarm {
    /// Whether the alarm function of the CUAS system is active or not.
    pub active: bool,
    /// How certain is the system of an active alarm, as a value from 0 (Least likely) to 1 (Most likely).
    #[cfg_attr(feature = "schemars", validate(range(min = 0., max = 1.)))]
    pub certainty: f64,
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
/// Local cartesian coordinates given in East, North and Up using WGS84.
pub struct CoordENU {
    /// Component tangent to the parallels. (Positive = east, negative = west)
    pub east: f64,
    /// Component to meridians. (Positive = north, negative = south)
    pub north: f64,
    /// Component perpendicular to the WGS84 ellipsoid. (Positive = up, negative = down)
    pub up: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
/// Location of an UAS, which may be relative to the CUAS.
#[cfg_attr(
    feature = "schemars",
    schemars(description = "Location of an UAS, which may be relative to the CUAS.
Location objects are composed of a single key which indicates the variant being used and a content element associated with it which contains the variant's data.")
)]
pub enum Location {
    // clockwise: from -> to (degrees)
    /// Circular arc relative to the CUAS within which the UAS resides.
    #[serde(rename = "arc")]
    Arc(Arc),
    /// Compass quadrant where the UAS has been observed.
    #[serde(rename = "quad")]
    Quad(Quad),
    // clockwise from true north (degrees)
    /// Clockwise angle in degrees from true north where the UAS has been observed.
    #[serde(rename = "bearing")]
    Bearing(f64),
    /// Flat 2D position given in latitude and longitude.
    #[serde(rename = "position2d")]
    Position2d(Position2d),
    /// 3D position given in latitude, longitude and height.
    #[serde(rename = "position3d")]
    Position3d(Position3d),
    /// Ray where the UAS has been observed given in bearing and elevation.
    #[serde(rename = "bearing_ele")]
    BearingElevation {
        /// Clockwise angle in degrees from true north where the UAS has been observed.
        bearing: f64,
        /// Elevation angle in degrees over the horizon where the UAS has been observed.
        elevation: f64,
    },
    /// 3D position of the UAS given in bearing, elevation angle and distance.
    #[serde(rename = "bearing_ele_dist")]
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
/// 3D (WGS84 horizontal datum EGM96 vertical datum) position given in latitude, longitude and height.
pub struct Position3d {
    /// GPS WGS84 latitude measured in degrees.
    pub lat: f64,
    /// GPS WGS84 longitude measured in degrees.
    pub lon: f64,
    /// Height measured in meters from mean sea level (EGM96).
    #[serde(rename = "height_amsl")]
    pub height: f64,
}

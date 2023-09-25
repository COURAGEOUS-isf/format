# 0.5.1+schema.0.3.3
- Add `uav_home_location` member to detection sets and tracks.

# 0.4.1+schema.0.3.2
- Update and clarify documentation
- Specify that height is respect to the EGM96 geoid. This was previously assumed; The wording used in the documentation has changed to clarify this further.

# 0.4.0+schema.0.3.1
- Added optional velocity member

# 0.3.1+schema.0.3.0
- Fix `height_amsl` serde, which was being parsed as `height` if the `schemars` feature was not enabled

# 0.3.0
- Improve docs
- Replace `alarm` and `alarm_certainty` members with single `alarm` member that contains both and is optional on detection
- Rename `height` to `height_amsl` on `Position3d`s
- Add `version` member, which must be equal to the schema version used
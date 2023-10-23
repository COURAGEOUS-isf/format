# 0.6.0+schema.0.4.0
- Rename `location` tag member from `t` to `$type`, which is a bit more explicit on its meaning.
- Use [internally tagged enum representation](https://serde.rs/enum-representations.html) for `location` members, which effectively gets rid of the `c` member, flattening its contents into the parent structure.
This means that what previously was represented as
```json
"t": "Position3d",
"c": {
    "lat": 0.1234,
    "lon": 0.1234,
    "height_amsl": 0.1234
}
```
Is now represented as
```json
"$type": "Position3d",
"lat": 0.1234,
"lon": 0.1234,
"height_amsl": 0.1234
```
For more details, check the schema documentation.
- Separate detection and tracking records as different types, for more flexibility on what either can contain.
- Enforce the `alarm` member on tracking records. Leaving it as null on tracking records previously resulted in implementation defined behavior; This is no longer the case.
- Rename `alarm_certainty` in `alarm` members to `certainty`. This was an oversight on 0.3.0 and thus has been fixed.
- Make the `location` member optional on detection records, as some C-UAS systems may sometimes detect UAS systems and some of their details but not know their exact location.


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
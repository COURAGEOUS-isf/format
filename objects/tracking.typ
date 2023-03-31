== Tracking <Tracking>
=== Syntax
```json
{
    "tracks": [ Track ],
    "altitude_mode": "AMSL" OR "AGL",
    <optional> "extensions": map
}
```

=== Members
==== tracks
A list of tracks. See @Track.

==== altitude_mode
What altitude values belonging to this object are relative to. Has only two valid values:
- `AMSL`, for height above mean sea level.
- `AGL`, for height above ground level.

== Track <Track>
=== Syntax 
```json
{
    "segments": [ TrackSegment ],
    <optional> "name": string,
    <optional> "extensions": map
}
```

=== Members
==== segments
A list of track segments (See @TrackSegment) detailing the trajectory of the object over time as seen by the radar.

==== name
Name of drone track. Optional. The value is arbitrary, but it must be a string.


== TrackSegment <TrackSegment>
=== Syntax
Either
```json
{
    "points": [ TrackPoint ],
}
```
or
```json
{
    "rays": [ Ray ],
}
```


== TrackPoint
=== Syntax
```json
{
    "coords": Coordinates,
    "time": string,
    <optional> "identification": string,
    <optional> "extensions": map,
}
```

=== Members
==== coords
The coordinates of the point.

==== time
Time linked to this point in UTC, written using the #link("https://tools.ietf.org/html/rfc3339#section-5.6", "RFC3339") standard.

=== identification
An arbitrary string containing identification of the object, e.g. "UAV", "Vehicle", "Person", etc. Optional.

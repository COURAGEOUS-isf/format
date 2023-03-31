== Detection <Detection>
=== Syntax 
```json
{
    "locations": [ DetectionLocation ],
    "altitude_mode": "AMSL" OR "AGL",
    <optional> "extensions": map
}
```

=== Members
==== locations
A list of disjoint detection locations. See @DetectionLocation.

==== altitude_mode
What altitude values belonging to this object are relative to. Has only two valid values:
- `AMSL`, for height above mean sea level.
- `AGL`, for height above ground level.


== DetectionLocation <DetectionLocation>
=== Syntax 
Either
```json
{
    "point": Coordinate,
    "time": string,
    <optional> "identification": string,
    <optional> "extensions": map
}
```
or
```json
{
    "ray": Ray,
    <optional> "identification": string,
    <optional> "extensions": map
}
```

=== Members
==== point
Indicates where the object was detected. See @Coordinates.

==== ray
Indicates where the object was detected. See @Ray. In contrast to `point`, this value might be either a point (`Ray` with a set `distance`) or a semiline (`Ray` with no set distance).

==== time
Time linked to this point in UTC, written using the #link("https://tools.ietf.org/html/rfc3339#section-5.6", "RFC3339") standard.

==== identification
An arbitrary string containing identification of the object, e.g. "UAV", "Vehicle", "Person", etc. Optional.

== Coordinates <Coordinates>
=== Syntax

```json
{
    "value": Quad OR Bearing OR Position OR 3DPosition,
    "type": "Quad" OR "Bearing" OR "Position" OR "3DPosition"
}
```
// type is the Declaration Type.
```json
{
    "lat": number,
    "long": number,
    "alt": number
}
```



=== Members
==== lat
Latitude of the coordinate in decimal degrees.

==== long
Longitude of the coordinate in decimal degrees.

==== alt
Altitude of the coordinates in meters. Altitude mode is given by the context.


== Ray <Ray>
// missing ray type origin,azimuth,elevation,distance (point rather than ray)
=== Syntax
```json
{
    "origin": coordinate,
    "azimuth": number,
    "elevation": number,
    "time": string, 
    <optional> "distance": number
}
```
=== Members
==== origin
Coordinates of the origin of the ray.

==== azimuth
Clockwise azimuth angle in decimal degrees, with 0 degrees being true north.

==== elevation 
Elevation angle from surface in decimal degrees, with 0 degrees being tangent to the surface and 90 degrees being perpendicular to it.

==== time
Time linked to this point in UTC, written using the #link("https://tools.ietf.org/html/rfc3339#section-5.6", "RFC3339") standard.

==== distance
If this member is skipped, the object refers to a geometric semi-line starting from `origin`. Otherwise, it refers to the point at that line that is `distance` meters away from the origin.
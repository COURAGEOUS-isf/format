#import "template.typ": *

#show: project.with(
  title: "COURAGEOUS Provisional Format",
  authors: (
    "Alejandro Perea Bonilla",
    "Marco Castell Martín",
  ),
)

#set heading(numbering: "1.")
#show link: set text(blue)
#show raw.where(block: true): block.with(fill: rgb("#edf4ff"), inset: 10pt, radius: 4pt, width: 100%, breakable: false)

#show raw.where(block: false): box.with(fill: rgb("#edf4ff"), inset: (x: 3pt, y: 0pt), outset: (y: 3pt), radius: 2pt,)

#outline(depth: 2, indent: true)

= Object notation used
We felt that we required a structured format as opposed to e.g. a table, as data scopes can vary. For instance, the version of the format is global to the document, whereas the elevation of a point is specific to a single data point. Using a structured format also allows easily extending it without breaking backwards compatibility.

We chose #link("https://www.json.org/json-en.html", "JSON") due to its simplicity and amount of libraries available for writing and parsing data.

= Notation used
// ray, ray origin, segment(vec<vec<trackpoint>>), time UTC ONLY, track, detection point, tracking, detection
- *Ray*: A geometrical half-line, starting at an origin point, and with a given direction extending to infinity.
- *Track*: An information structure containing position data of an object detected by the radar over time, as well as possibly information about that object.
- *Track segment*: A list of points belonging to a track that are meant to be visually connected together, because the radar has not lost sight of the object in between them.
- *Trajectory*: An information structure containing position data of an object detected by the radar over time.
- *Altitude Mode*: Indicates what an altitude value is relative to: either ground or sea level.


= Extensions
Most elements in the document structure present an `extensions` member. This is a map object meant to allow extending the format if required, by associating new values with an _extension name_.

Extension names have the format `ISSUER_NAME`, where `ISSUER` is the name of the entity introducing the extension (e.g. COURAGEOUS or any other company using the format) and `NAME` is an arbitrary string which may only contain characters as specified by the #link("https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%3AXID_Continue%3A%5D&abb=on&g=&i=", [`XID_Continue`]) unicode set of characters. The value associated with the key is determined by the extension's definition, and can be of any type.

= Document structure
#include "schema_reader.typ"

= Notes & TODOs
- The inclusion of rays is not very elegant, their name is not ideal, and they can actually refer to two possible geometric terms of location (Resulting in either a point or a ray)
- We need to include more data on track points, such as the speed, velocity, quad and bearing of the UAV. All of these should be left as optional.
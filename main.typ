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

= Format specification
The format is specified in a #link("https://json-schema.org/", "JSON Schema"). // TODO: Provide link to json schema

#let url_encode(s) = {
  let result = ""
  for char in s {
    if "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~".contains(char) {
      result = result + char
    } else {
      result = result + (
        "{": "%7B",
        "}": "%7D",
        "\n": "%0A",
        " ": "%20",
        "\"": "%22",
        "/": "%2F",
        "#": "%23",
        ",": "%2C",
        "[": "%5B",
        "]": "%5D",
        "(": "%28",
        ")": "%29",
        "?": "%3F",
        "+": "%2B",
        "\\": "%5C",
        "^": "%5E",
        "'": "%27",
        "$": "%24",
        ":": "%3A",
      ).at(char)
    }
  }
  result
}

// TODO (VERY IMPORTANT): Change URL
A visualization of the format is available on #link("https://aleokdev.github.io/json-schema-visualizer/?hideEditor&maxLevel=9999&surl=https://raw.githubusercontent.com/aleokdev/json-schema-visualizer/test/courageous.schema.json", "this page").
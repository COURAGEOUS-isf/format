== Document <Document>
=== Syntax
```json
{
    "version": "1.0",
    <optional> "tracking": Tracking,
    <optional> "detection": Detection,
    <optional> "extensions": map
}
```

=== Members
==== version
Version of the format used. Only valid value for now is `1.0`.

==== tracking
The (optional) tracking data of the document. See @Tracking.

==== detection
The (optional) detection data of the document. See @Detection.

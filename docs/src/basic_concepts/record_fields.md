## Record organization
The format uses different structures to organize and distinguish data.

### Records
**Records** represent all the information related with an entity in a given time
and, when applicable, point in space.

### Record sets
**Record sets** are clusters of records under a unique `uas_id` and `name`.
These fields are used to identify and distinguish record sets within the format.

- `name` simply acts as free-form text to distinguish record sets. 
As it is an optional field, each company can use it to organize their data
as they see fit.

- `uas_id` is used to distinguish and correlate different record sets within the format.
Unique sets of records should have unique values while record sets that refer to 
the same UAS should have the same `uas_id`. In the case of tracks, this allows a method 
of logging the C-UAS losing the UAS momentarily, emulating a discontinuous track.

It is **essential** for **C-UAS that intercept the UAS communication** to view the 
[Intercepting C-UAS segment](../common_uses/intercepting.md) to understand the
UAV - GCS logging procedure. 

[^1]: Record sets refers to both track and detection set elements (inside `detection` and `tracks`).
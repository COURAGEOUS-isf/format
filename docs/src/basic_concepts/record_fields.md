## Record organization
The format uses various fields to organize and distinguish **records**
and **record sets** [^1]:
`name`, `uas_id` and `record_number`.

### Records
**Records** are used to store data both in **detection** as well as **tracking**.
Each record must have a unique `record_number` inside **detection** or **tracking**,
meaning that the same `record_number` can appear in both without causing conflict.

### Record sets
**Record sets** are the elements of `tracks` and `detection`.
They contain clusters of records under a unique `uas_id` and `name`.
These fields are used to identify and distinguish record sets within the format.

- `name` simply acts as free-form text to distinguish **record sets**. 
As it is an optional field, each company should use it to organize their data
as they see fit.

- `uas_id` is used to distinguish and relate different **record sets** within the format.
Unique sets of records should have unique values while record sets that refer to 
the same UAS should have the same `uas_id`. 

In the case of tracks, this allows a [method of logging the C-UAS losing the UAS momentarily](detec_track.md),
creating a discontinuous track.

It is **essential** for **C-UAS that intercept the UAS communication** to view the 
[Intercepting C-UAS segment](../common_uses/intercepting.md) to understand the
UAV - GCS logging procedure. 

----
[^1] Record sets refers to both track and detection set elements (inside `detection` and `tracks`).
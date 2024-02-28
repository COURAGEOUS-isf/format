## Intercepting C-UAS

Systems that intercept the communication between GCS and UAV require a 
specific logging procedure.

### 1. UAV logging
When a UAV is detected or tracked, the system should log the UAV position
instances in the same record set[^1] under a unique `uas_id`.

### 2. GCS logging
The GCS location can be logged either in the same record set[^1] or a different one with the same
`uas_id`.

### 3. UAV Home location
In the case of C-UAS systems that are capable of extracting the **home location** of the UAV,
it should be included in the UAV record set[^1] using the `uav_home_location` member.

### Example

This is set of records extracted from the [intercepting C-UAS COURAGEOUS format example](../res/example_intercepting.json).

```json
...
 {
    "classification": "UAV",
    "identification": "UAV Description",
    "alarm": {
        "active": true,
        "certainty": 1
    },
    "location": {
        "$type": "Position3d",
        "height_amsl": 52.33707466137043,
        "lat": 51.15836795030467,
        "lon": 2.736868683313011
    },
    "record_number": 2,
    "time": 1696421607569
},
{
    "classification": "GCS",
    "identification": "GCS Description",
    "alarm": {
        "active": true,
        "certainty": 1
    },
    "location": {
        "$type": "Position3d",
        "height_amsl": 9,
        "lat": 51.157409,
        "lon": 2.740082
    },
    "record_number": 3,
    "time": 1696421411207
},
...
```

[^1]: detection set or track (members of `detection`or `tracks`, respectively).

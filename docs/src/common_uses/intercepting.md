## Intercepting C-UAS

Systems that intercept the communication between GCS and UAV require a 
specific logging procedure.

**Logging both the GCS and the UAV under the same detection set/track is an error
and will make such data unusable**.

### 1. UAV logging
When a UAV is detected or tracked, the system should log the UAV position
instances in the same record set[^1] under a unique `uas_id`.

### 2. GCS logging
The GCS location must be logged under a different record set[^1], but should
contain the same `uas_id` as the UAV records, so as to link both.

### 3. UAV Home location
In the case of C-UAS systems that are capable of extracting the **home location** of the UAV,
it should be included in the UAV record set[^1] using the `uav_home_location` member.

[^1]: detection set or track (members of `detection`or `tracks`, respectively).

## Intercepting C-UAS

Systems that intercept the communication between GCS and UAV require a 
specific logging procedure.

Logging both the GCS and the UAV under the same detection set/track is an **error
and will make such data unusable**.

### 1. UAV logging
When a UAV is detected or tracked, the system should log the UAV position
instances in the same **record set[^1]** under a unique `uas_id`.

### 2. GCS logging
The GCS location must be logged under a different **record set[^1]**, but should
contain the same `uas_id` as the uav records, so as to link both.

### 3. UAV Home location
In the case the C-UAS is able to extract the **home location** of the uav,
it should be included in the uav **record set[^1]**.

----
[^1] detection set or track
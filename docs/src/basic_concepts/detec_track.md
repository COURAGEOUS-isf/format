## Tracking and Detection
The format distinguishes 2 types of data: **tracking** and **detection**.
Data inside `tracks` and `detection` is organized in record sets
(called *tracks* and *detection sets*, respectively).

### Tracking
**Tracking** is used for targets that the C-UAS has locked on and is 
actively tracking their positions.
These records should describe the trajectory of a specific target. 

> When the system locks on a target and starts to track its position, the system should
log each continuous track as an element of `tracks`. 
In the case of a target coming in and out of view, each continuous track should be logged
separately under the same `uas_id`, as a way to link the tracks.

### Detection
**Detection** is used for sporadic detections without a clear spatial or 
temporal relation and thus can't be considered to be tracked. 
This does not include tracks that are momentarily lost and picked up later.  
Targets aren't required to be identified to be part of a record, only detected.

> When these systems detect potential targets, the system should log each 
unique target as a **different detection set** (members of `detection`).
In the case of entities not properly identified as a unique target, they should
omit the `uas_id` parameter.
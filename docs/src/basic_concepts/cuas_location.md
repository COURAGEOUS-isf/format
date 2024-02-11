## C-UAS Location

C-UAS systems are required to provide their position for the trials.
To accommodate for mobile and static systems, there are 2 methods of
providing the C-UAS position.

`static_cuas_location` is used for denoting the position of a static
system. This field is **compulsory for all C-UAS**, but mobile
systems can rely on the `cuas_location` field inside all records for 
a dynamic position that will override the value given on `static_cuas_location`.


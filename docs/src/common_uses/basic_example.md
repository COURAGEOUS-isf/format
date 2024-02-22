## Basic example


Here are record examples extracted from the [basic COURAGEOUS example file]. ###################################link here to example_basic.json

```
...
{
    "alarm": {
        "active": true,
        "certainty": 1
    },
    "classification": "Unknown",
    "location": {
        "$type": "BearingElevation",
        "bearing": 43.563024999999996,
        "elevation": 2.211875
    },
    "record_number": 4,
    "time": 1696255068480
},
...
```

```
...
{
    "classification": "UAV",
    "identification": "UAV description",
    "alarm": {
        "active": false,
        "certainty": 0.55
    },
    "location": {
        "$type": "Position3d",
        "height_amsl": 47.15314324164956,
        "lat": 51.158929232973726,
        "lon": 2.7414165702140973
    },
    "record_number": 0,
    "time": 1696421411207,
    "velocity": {
        "east": -2.14297,
        "north": 1.69235,
        "up": -0.86349
    }
},
...
```
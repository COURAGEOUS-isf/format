## Classification and Identification
The format allows systems to classify and identify targets through the
appropriately named fields `classification` and `identification`.
The only required field is the `classification` while the `identification` 
field is used to complement the classification.

**Classification** refers to the basic category of the target detected.
The available options are:
- **Unknown**: The C-UAS could not classify the entity
- **UAV**: Unmanned Aerial Vehicle
- **GCS** : Ground Control Station, the entity is the operator of another entity of the same `uas_id`
- **Other**: The entity doesn't fall in any of the previous categories, e.g. _bird_.

**Identification** refers to a more detailed description of the entity in question. 
This is meant to act as a more concise description of the entity than **classification**.
As such, it is left free for the company to *include a description more in line with their 
system's capabilities*, whether it's specific information on the target (operator’s ID, 
ID of the drone, UAV model, etc) or a more robust classification than the one provided.
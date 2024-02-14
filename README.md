# WRU

Sending input pictures/files -> receiving + storing them -> processing them -> displaying meaningful output.

READMEs of each section:<br />
[Sending input pictures/files (field_scout)](/field_scout/README.md)<br />
[recieving + storing them (file_intake)](/file_intake/README.md)<br />
[processing them (processing)](/processing/README.md)<br />
[displaying meaningful output (result)](/result/README.md)<br />

Still TODO:
- **processing** must perform object, person, and animal recognition, not just facial. *(Medium)*
- **processing** must save results to a DB. *(Medium)*
- **field_scout** must be able to send images that already exist in the operating folder, not just live created ones (so that the program may be down for any amount of time and still recover by sending the images later when restarted). *(Small)*
- **field_scout** should delete images that are confirmed to have been uploaded. *(Small)*
- **field_scout** should be able to operate in a Meshtastic-only mode. *(Large)*

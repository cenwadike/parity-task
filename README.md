# Patient Biodata

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) 

This pallet supports creating record of patient data.  
It also allow patients to grant or revoke access to their biodata for a given account.

## Interface

### Config

### Dispatchable functions

* `create_new_record(patient_id, name, age, sex)` 
   Create a new patient record. patient_id is Origin.
* `grant_access(patient_id, new_access_id, record_id)` 
   Grant access to patient record to new_access_id. Only patient can grant access to their data
* `grant_access(patient_id, access_id, record_id)` 
   Revoke access to patient record from access_id. Only patient can revoke access to their data


## Planned features

- [ ] Make data modifiable
- [ ] Introduce different state of data (AVAILABLE, WITHDRAWN, NONEXISTANT)
- [ ] Introduce timed access to data
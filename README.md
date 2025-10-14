# Ground-Trace

This project is a satellite telemetry hub that will visualize and update with current telemetry from multiple sattelites. Telemetry data is pulled from public APIs, ingested, decoded, and visualized in one location. The core ingesting, decoding, and reformatting is done in rust, and will be stored using PostreSQL.

## Phase 0

- [1] Model test data from available public satellite telemetry.
- [2] Create Postgres db for ingestion and decoding testing.
- [3] Initialize docker container to store testing data

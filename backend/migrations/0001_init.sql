CREATE TABLE satellite (
  sat_id TEXT PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE packet (
  id BIGSERIAL PRIMARY KEY,
  sat_id TEXT NOT NULL REFERENCES satellite(sat_id),
  ts_ms BIGINT NOT NULL,
  rssi_db REAL,
  snr_db REAL,
  raw BYTEA
);

CREATE TABLE telemetry (
  id BIGSERIAL PRIMARY KEY,
  sat_id TEXT NOT NULL REFERENCES satellite(sat_id),
  ts_ms BIGINT NOT NULL,
  key TEXT NOT NULL,
  value DOUBLE PRECISION NOT NULL
);

CREATE INDEX idx_tel_sat_ts ON telemetry (sat_id, ts_ms);
CREATE INDEX idx_pkt_sat_ts ON packet (sat_id, ts_ms);

INSERT INTO satellite (sat_id, name) VALUES
  ('ISS', 'International Space Station'),
  ('CUBE1', 'Demo CubeSat 1')
ON CONFLICT DO NOTHING;

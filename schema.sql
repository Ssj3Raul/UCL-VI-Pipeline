DROP TABLE IF EXISTS reading;
DROP TABLE IF EXISTS sensor;
DROP TABLE IF EXISTS iotSource;
DROP TABLE IF EXISTS iotFlow;


CREATE TABLE iotFlow (
    id           UUID PRIMARY KEY,
    name         TEXT NOT NULL,
    nodes        JSONB NOT NULL, -- array of node definitions (sources/processors)
    edges        JSONB NOT NULL, -- array of how nodes are connected
    createdAt    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updatedAt    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE iotSource (
    id           UUID PRIMARY KEY,
    siteId       TEXT NOT NULL,
    name         TEXT NOT NULL,
    description  TEXT,
    type         TEXT NOT NULL,        -- 'MQTT', 'HTTP', 'MAS_MONITOR', etc.
    config       JSONB NOT NULL        -- source-specific config (host, topic, credentials, etc.)
);

CREATE TABLE sensor (
    id           UUID PRIMARY KEY,
    identifier   TEXT NOT NULL,
    measuring    TEXT NOT NULL,   -- e.g. 'temperature'
    unit         TEXT             -- e.g. 'C'
    -- (other metadata as needed)
);

CREATE TABLE reading (
    id           UUID PRIMARY KEY,
    sensorId     UUID NOT NULL REFERENCES sensor(id),
    timestamp    TIMESTAMPTZ NOT NULL,
    rawValue     DOUBLE PRECISION,
    value        DOUBLE PRECISION,
    createdAt    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

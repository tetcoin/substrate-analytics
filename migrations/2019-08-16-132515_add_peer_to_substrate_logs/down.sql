ALTER TABLE tetcore_logs DROP COLUMN peer_connection_id;
ALTER TABLE tetcore_logs ADD COLUMN node_ip VARCHAR NOT NULL DEFAULT 'NULL';
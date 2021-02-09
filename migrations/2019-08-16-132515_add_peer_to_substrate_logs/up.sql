TRUNCATE tetcore_logs;
ALTER TABLE tetcore_logs DROP COLUMN node_ip;
ALTER TABLE tetcore_logs ADD COLUMN peer_connection_id INTEGER REFERENCES peer_connections(id) NOT NULL;
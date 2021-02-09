CREATE INDEX tetcore_logs_peerset_nodes_idx ON tetcore_logs USING GIN ((logs->'state'->'peerset'->'nodes') jsonb_path_ops);

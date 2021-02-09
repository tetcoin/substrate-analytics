table! {
    benchmark_events (id) {
        id -> Int4,
        benchmark_id -> Int4,
        name -> Varchar,
        phase -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    benchmarks (id) {
        id -> Int4,
        setup -> Jsonb,
        created_at -> Timestamp,
    }
}

table! {
    host_systems (id) {
        id -> Int4,
        description -> Varchar,
        os -> Varchar,
        cpu_qty -> Int4,
        cpu_clock -> Int4,
        ram_mb -> Int4,
        disk_info -> Varchar,
    }
}

table! {
    peer_connections (id) {
        id -> Int4,
        ip_addr -> Varchar,
        peer_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        audit -> Bool,
        name -> Nullable<Varchar>,
        chain -> Nullable<Varchar>,
        version -> Nullable<Varchar>,
        authority -> Nullable<Bool>,
        startup_time -> Nullable<Int8>,
        implementation -> Nullable<Varchar>,
    }
}

table! {
    tetcore_logs (id) {
        id -> Int4,
        created_at -> Timestamp,
        logs -> Jsonb,
        peer_connection_id -> Int4,
    }
}

joinable!(benchmark_events -> benchmarks (benchmark_id));
joinable!(tetcore_logs -> peer_connections (peer_connection_id));

allow_tables_to_appear_in_same_query!(
    benchmark_events,
    benchmarks,
    host_systems,
    peer_connections,
    tetcore_logs,
);

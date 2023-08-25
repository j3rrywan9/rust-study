create table test_run (
    test_run_id uuid primary key default gen_random_uuid(),
    build_number text not null,
    build_url text,
    build_timestamp timestamptz not null default now()
);

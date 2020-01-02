create sequence global_id_sequence;

create or replace function id_generator(out result bigint) as
$$
declare
    our_epoch  bigint := 1577836800000;
    seq_id     bigint;
    now_millis bigint;
    -- the id of this db shard, must be set for each
    -- schema shard you have - you could pass this as a parameter too
    shard_id   int    := 1;
begin
    select nextval('global_id_sequence') % 1024 into seq_id;

    select floor(extract(epoch from clock_timestamp()) * 1000) into now_millis;
    result := (now_millis - our_epoch) << 23;
    result := result | (shard_id << 10);
    result := result | (seq_id);
end;
$$ language plpgsql;


create or replace function timestamp_guard() returns trigger as
$$
begin
    raise check_violation using constraint = 'edit_timestamp';
end;
$$ language plpgsql;

select id_generator();

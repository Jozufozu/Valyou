create type status as enum ('pending_first_second', 'pending_second_first', 'friends', 'block_first_second', 'block_second_first', 'block_both');

create or replace function are_public(user1 bigint, user2 bigint) returns boolean as $$
begin
    return (
               select count(1) from profiles p
               where ((p.userid=user1 or p.userid=user2) and p.visibility != 'private')
           ) = 2;
end;
$$ language plpgsql;

create table relations (
    user_from   bigint      not null    references profiles on update cascade on delete cascade,
    user_to     bigint      not null    references profiles on update cascade on delete cascade,
    status      status      not null,
    since       timestamp   not null    default now(),

    constraint  id_order    check ( user_from < user_to ),
    constraint  friend_self check ( user_from != user_to ),
    constraint  are_public  check ( are_public(user_from, user_to) ),
    primary key (user_from, user_to)
);

create view public_friends as
    select self as userid, friend, p.username, p.discriminator, p.summary, p.bio, since from (
        select self, friend, since from (
            select user_to as self, user_from as friend, status, since
            from relations
            union
            select user_from as self, user_to as friend, status, since
            from relations
        ) as f
        where f.status='friends'
    ) as f
    inner join searchable p on f.friend=p.userid;

create view friend_requests as
    select distinct on (self, friend) self as userid, friend, p.username, p.discriminator, p.summary, p.bio, since from (
        select user_to as self, user_from as friend, status, since
        from relations
        where status='pending_first_second'
        union
        select user_from as self, user_to as friend, status, since
        from relations
        where status='pending_second_first'
    ) as f
    inner join searchable p on f.friend=p.userid;

create or replace function cascade_private() returns trigger as $$
begin
    delete from relations
    where user_to=new.userid and status='pending_first_second';

    delete from relations
    where user_from=new.userid and status='pending_second_first';

    return new;
end;
$$ language plpgsql;

create trigger cascade_private
after update
on profiles
for each row
when ( new.visibility='private' )
execute procedure cascade_private();

create type visibility as enum ('public', 'private', 'friends');

create table profiles (
    userid          bigint  primary key references accounts on update cascade on delete cascade,
    visibility  visibility  not null    default 'private',
    summary     varchar(120),
    bio         varchar(400),
    modified    timestamp
);

create table usernames (
    userid              bigint      primary key references profiles on update cascade on delete cascade,
    username        varchar     not null,
    discriminator   smallint    not null    check ( discriminator < 10000 and discriminator > 0 ),
    modified        timestamp,

    unique (username, discriminator)
);

create view searchable as
    select u.userid, u.username, u.discriminator, p.summary, p.bio from profiles p
    inner join usernames u on p.userid = u.userid
    where p.visibility!='private';
drop view friend_requests;
drop view public_friends;
drop table relations;

drop trigger cascade_private on profiles;
drop function if exists are_public;
drop function if exists cascade_private;
drop type status;
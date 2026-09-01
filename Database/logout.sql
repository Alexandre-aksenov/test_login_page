DROP procedure IF exists logout;


create or replace procedure logout(session_id int, user_id int, session_hash VARCHAR) 
language plpgsql
SECURITY definer -- makes the proc callable from the game, although it does not have the rights for individual UPDATE
as $$
declare
	sess record;
	session_exists boolean;
begin
	-- check the session exists and is active
	session_exists := sess_exists(
		session_id => logout.session_id,
		u_id => logout.user_id,
		session_hash => logout.session_hash);	

	-- end session
	if session_exists then
		UPDATE connections c
		set end_session = current_timestamp
		WHERE c.connection_id = session_id AND c.user_id = logout.user_id AND c.session_hash_key = session_hash;
	else
		raise exception 'Attempt to logout from an ended or inexisting session';
	end if;
end;
$$;
-- CREATE PROCEDURE


-- test (as the user 'game') after testing other procedures

-- (wrong session_id)
call logout(session_id => 2,
	user_id => 1, 
	session_hash => '53388382b1548a525935ff1872082f70fa5efb46ae04185f2b41edc1dc30ba0c'::VARCHAR);
-- ERROR (expected):  Attempt to logout from an ended or inexisting session


call logout(session_id => 1,
	user_id => 1, 
	session_hash => '53388382b1548a525935ff1872082f70fa5efb46ae04185f2b41edc1dc30ba0c'::VARCHAR);
-- CALL

select * from connections;
/*

 connection_id | user_id |                         session_hash_key                         |         start_session         |          end_session          
---------------+---------+------------------------------------------------------------------+-------------------------------+-------------------------------
             1 |       1 | 53388382b1548a525935ff1872082f70fa5efb46ae04185f2b41edc1dc30ba0c | {time of start of testing}    | {time of end of testing}
(1 row)

The times are printed in the format: yyyy-mm-dd hh:mm:ss.sssss+00
*/

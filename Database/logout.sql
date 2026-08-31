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

/*
sess_exists := False;
for sess in -- table of <=1 row
	SELECT *
	FROM  connections c
	WHERE c.connection_id = session_id AND c.user_id = logout.user_id AND c.session_hash_key = session_hash
loop
	sess_exists := (sess.end_session is NULL);
end loop;
*/

-- test (from user 'game') after testing of other procedures


call logout(session_id => 2,
	user_id => 1, 
	session_hash => '527fb4d903e7c614299f8a91eec7a02e2e306bfb54cbd399cc52d46b22bc0284'::VARCHAR);
-- CALL

select * from connections;
/*

 connection_id | user_id |                         session_hash_key                         |         start_session         |          end_session          
---------------+---------+------------------------------------------------------------------+-------------------------------+-------------------------------
             2 |       1 | 527fb4d903e7c614299f8a91eec7a02e2e306bfb54cbd399cc52d46b22bc0284 | 2026-08-30 23:08:34.115934+00 | 2026-08-31 09:46:27.849172+00
(1 row)


*/

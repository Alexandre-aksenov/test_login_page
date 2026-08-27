
DROP procedure logout;

create or replace procedure logout(session_id int)
language plpgsql
SECURITY definer -- makes the proc callable from the game, although it does not have the rights for individual SELECT, UPDATE
as $$
declare
	sess record;
	sess_exists boolean;
begin
	-- check the session exists and is active
	sess_exists := False;
	for sess in -- table of <=1 row
		SELECT *
		FROM  connections c
		WHERE c.connection_id = session_id
	loop
		sess_exists := (sess.end_session is NULL);
	end loop;

	-- end session
	if sess_exists then
		UPDATE connections 
		set end_session = current_timestamp
		WHERE connection_id = session_id;
	else
		raise exception 'Attempt to logout from an ended or inexisting session';
	end if;
end;
$$;
-- CREATE PROCEDURE

-- tests

call sign_in(login => 'default_user', pwd => 'default_pwd');
/*
NOTICE:  Logged in as account default_user , new connection id is 1
 new_connection_id 
-------------------
                 1
(1 row)

5.5 ms
*/


select * from connections;
/*

 connection_id | user_id |         start_session         | end_session 
---------------+---------+-------------------------------+-------------
             1 |       1 | 2026-08-11 16:00:51.661482+00 | 
(1 row)

Time: 1,268 ms
*/

call logout(session_id => 1);
-- 4.2 ms

select * from connections;
/*

 connection_id | user_id |         start_session         |          end_session          
---------------+---------+-------------------------------+-------------------------------
             1 |       1 | 2026-08-11 16:00:51.661482+00 | 2026-08-11 16:15:50.279856+00
(1 row)

Time: 0.9 ms
*/

-- This fn checks whether a session with given parameters exists and is active

drop function if exists sess_exists(INTEGER, INTEGER, VARCHAR);

create or replace function sess_exists(
	session_id int, u_id int, session_hash VARCHAR
)
returns boolean
language plpgsql 
as $$
DECLARE
	sess_exists boolean;
	sess record;
BEGIN
	sess_exists := False;

	for sess in -- table of <=1 row
		SELECT end_session
		FROM  connections c
		WHERE c.connection_id = session_id AND c.user_id = u_id AND c.session_hash_key = session_hash
	loop
		sess_exists := (sess.end_session is NULL);
	end loop;
	return sess_exists;
END;
$$;
-- CREATE FUNCTION


-- test, call with the user admin
SELECT sess_exists(session_id => 1, u_id => 1, session_hash => '53388382b1548a525935ff1872082f70fa5efb46ae04185f2b41edc1dc30ba0c'::VARCHAR);
/*
 sess_exists 
-------------
 t
(1 row)

 */

SELECT sess_exists(session_id => 1, u_id => 1, session_hash => 'ff'::VARCHAR);
/*
 sess_exists 
-------------
 f
(1 row)

 */


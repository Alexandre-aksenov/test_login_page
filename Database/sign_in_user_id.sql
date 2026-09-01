-- Procedure to login, with 3 outputs:
-- new_connection_id, user_id, new_session_hash .
-- This procedure allows creating 2 parallel sessions in different tabs of the browser


DROP procedure IF exists sign_in_user_id;

create or replace procedure sign_in_user_id(
	login VARCHAR(255)
	, pwd VARCHAR(255)
	, inout new_connection_id int  DEFAULT 0 -- used only as the output
	, inout user_id int  DEFAULT 0
	, inout new_session_hash VARCHAR default '' 
	)
language plpgsql
SECURITY definer
as $$
declare
	id_exists boolean;
	stored_pwd record;
	pwd_verified boolean;
	u_id INTEGER;
	session_hash_in_loop VARCHAR;
begin
  	-- check that the user exists
	id_exists := (EXISTS (
		SELECT *
		FROM  users u
		WHERE u.login = sign_in_user_id.login	
	));

	if id_exists then
		for stored_pwd in 	-- check the pwd, the loop should run just once
			SELECT
				u.user_id
				, u.pwd
			FROM users u
			WHERE u.login = sign_in_user_id.login
		loop
			u_id := stored_pwd.user_id;
			pwd_verified := (stored_pwd.pwd = sign_in_user_id.pwd);
			-- computation of hash

			-- now()::VARCHAR makes sure all sessions created by humans get different hashes
			session_hash_in_loop := encode(sha256(convert_to(sign_in_user_id.pwd || now()::VARCHAR || u_id, 'UTF8')), 'hex')::VARCHAR;
		end loop;

		if pwd_verified then
			-- add row to 'connections'
			INSERT INTO connections (user_id, session_hash_key, start_session, end_session) VALUES
				(u_id, session_hash_in_loop, current_timestamp, NULL);

			-- output values
			new_connection_id := currval('connections_connection_id_seq');
			user_id := u_id;
			new_session_hash := session_hash_in_loop;

			raise notice 'Logged in as account % , new connection id is %', login, new_connection_id;
			
		else
			raise exception 'Incorrect password';
		end if;
	else
		raise exception 'Login failed: no such account.';
	end if;
end;
$$;
-- Updated Rows 0

-- tests

--  query last 5 connections
SELECT * from connections
ORDER BY connection_id
LIMIT 5
OFFSET GREATEST((SELECT COUNT(*) FROM connections) -5, 0);
-- 0 rows


-- Try to sign in from the user 'game' with wrong password
call sign_in_user_id(login => 'default_user', pwd => 'ff');
-- ERROR (expected):  Incorrect password

call sign_in_user_id(login => 'default_user', pwd => 'deadbeef');
/*

 new_connection_id | user_id |                         new_session_hash                         
-------------------+---------+------------------------------------------------------------------
                 1 |       1 | 53388382b1548a525935ff1872082f70fa5efb46ae04185f2b41edc1dc30ba0c
(1 row)


*/

SELECT * from connections
ORDER BY connection_id
LIMIT 5
OFFSET GREATEST((SELECT COUNT(*) FROM connections) -5, 0);
-- 1 open session (end_session = NULL) with the previous connection_id, user_id, session_hash_key





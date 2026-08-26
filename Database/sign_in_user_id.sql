-- Procedure to login, with 2 outputs:
-- new_connection_id, user_id


DROP procedure IF exists sign_in_user_id;

create or replace procedure sign_in_user_id(
	login VARCHAR(255)
	, pwd VARCHAR(255)
	, inout new_connection_id int  DEFAULT 0 -- used only as the output
	, inout user_id int  DEFAULT 0 
	)
language plpgsql
as $$
declare
	id_exists boolean;
	stored_pwd record;
	pwd_verified boolean;
	u_id INTEGER;
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
			u_id = stored_pwd.user_id;
			pwd_verified := (stored_pwd.pwd = sign_in_user_id.pwd);
		end loop;

		if pwd_verified then
			-- add row to 'connections'
			INSERT INTO connections (user_id, start_session, end_session) VALUES
				(u_id, current_timestamp, NULL);
			new_connection_id := currval('connections_connection_id_seq');
			user_id := u_id;

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
-- last connection_id: 44




call sign_in_user_id(login => 'default_user', pwd => 'default_pwd');
/*
NOTICE:  Logged in as account default_user , new connection id is 45
 new_connection_id | user_id 
-------------------+---------
                45 |       1

*/

SELECT * from connections
ORDER BY connection_id
LIMIT 5
OFFSET GREATEST((SELECT COUNT(*) FROM connections) -5, 0);
-- 1 new open session, nb 45


call logout(session_id => 45);


SELECT * from connections
ORDER BY connection_id
LIMIT 5
OFFSET GREATEST((SELECT COUNT(*) FROM connections) -5, 0);
-- The new session is closed


call sign_in_user_id(login => 'default_user', pwd => 'wrong_pwd');
--  ERROR (expected):  Incorrect password


SELECT * from connections
ORDER BY connection_id
LIMIT 5
OFFSET GREATEST((SELECT COUNT(*) FROM connections) -5, 0);
-- (expected) Same as before: 45 rows



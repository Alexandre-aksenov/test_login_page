-- Proicedure to sign up a player on the server.
-- Adds a row to the table 'users'.
-- No check (yet) that the password meets the criteria.
-- In future, the password on the server will be the result of a hash (communicated by the client).

DROP procedure IF exists signup;

create or replace procedure signup(
	login VARCHAR(255),
	pwd VARCHAR(255)
)
language plpgsql
SECURITY definer
as $$
declare
	login_exists boolean;
begin
	-- check this login is unique
	login_exists := (EXISTS (
		SELECT *
		FROM  users u
		WHERE u.login = signup.login	
	));

	if login_exists then
		raise exception 'Please, find an original login';		
	else 	-- if not, add row to  the table 'users'.
		INSERT INTO users (login, pwd, registration_date, highest_level_won) VALUES
		(signup.login, signup.pwd, CURRENT_DATE, 0);
	end if;

end;
$$;
-- CREATE PROCEDURE



-- Tests (1) in psql

select * from users;
/*
 user_id |    login     |     pwd     | registration_date | highest_level_won 
---------+--------------+-------------+-------------------+-------------------
       1 | default_user | deadbeef 	  | 2026-08-30        |                 0
(1 row)

 */

-- Existing user. Should fail.
call signup(login => 'default_user', pwd => 'wrong_pwd');
-- ERROR:  Please, find an original login

select * from users;
/*
 user_id |    login     |     pwd     | registration_date | highest_level_won 
---------+--------------+-------------+-------------------+-------------------
       1 | default_user | deadbeef	  | 2026-08-11        |                 0
(1 row)
*/


-- New user.  Should work from the user 'game'.
call signup(login => 'test_user1', pwd => 'beef1234');
-- CALL

-- from admin user
select * from users;
/*
 user_id |    login     |     pwd     | registration_date | highest_level_won 
---------+--------------+-------------+-------------------+-------------------
       1 | default_user | deadbeef	  | 2026-08-30        |                 0
       2 | test_user1   | beef1234    | 2026-08-30        |                 0
(2 rows)

 */




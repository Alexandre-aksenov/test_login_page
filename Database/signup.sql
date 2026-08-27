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
SECURITY definer -- makes the proc callable from the game, although it does not have the rights for individual SELECT, INSERT
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
-- Existing user. Should fail.
select * from users;
/*
 user_id |    login     |     pwd     | registration_date | highest_level_won 
---------+--------------+-------------+-------------------+-------------------
       1 | default_user | default_pwd | 2026-08-11        |                 0
(1 row)

 */

call signup(login => 'default_user', pwd => 'wrong_pwd');
-- ERROR:  Please, find an original login

select * from users;
/*
 user_id |    login     |     pwd     | registration_date | highest_level_won 
---------+--------------+-------------+-------------------+-------------------
       1 | default_user | default_pwd | 2026-08-11        |                 0
(1 row)
*/


-- New user.  Should work.
call signup(login => 'test_user1', pwd => 'pwd1');
-- CALL

select * from users;
/*
 user_id |    login     |     pwd     | registration_date | highest_level_won 
---------+--------------+-------------+-------------------+-------------------
       1 | default_user | default_pwd | 2026-08-11        |                 0
       2 | test_user1   | pwd1        | 2026-08-12        |                 0
(2 rows)

 */



-- Login as this user.

call sign_in(login => 'test_user1', pwd => 'pwd1');
/*
NOTICE:  Logged in as account test_user1 , new connection id is 5
 new_connection_id 
-------------------
                 5

 */

select * from connections;
/*
 connection_id | user_id |         start_session         |          end_session          
---------------+---------+-------------------------------+-------------------------------
             1 |       1 | 2026-08-11 16:00:51.661482+00 | 2026-08-11 16:15:50.279856+00
             2 |       1 | 2026-08-11 22:02:37.3565+00   | 2026-08-11 22:04:43.956421+00
             3 |       1 | 2026-08-11 22:06:10.098199+00 | 2026-08-11 22:08:17.004238+00
             4 |       1 | 2026-08-11 22:08:30.010029+00 | 2026-08-11 22:08:30.014802+00
             5 |       2 | 2026-08-12 15:12:16.411774+00 | 
(5 rows)

 */

call logout(session_id => 5);
-- CALL

select * from connections;
/*
  connection_id | user_id |         start_session         |          end_session          
---------------+---------+-------------------------------+-------------------------------
             1 |       1 | 2026-08-11 16:00:51.661482+00 | 2026-08-11 16:15:50.279856+00
             2 |       1 | 2026-08-11 22:02:37.3565+00   | 2026-08-11 22:04:43.956421+00
             3 |       1 | 2026-08-11 22:06:10.098199+00 | 2026-08-11 22:08:17.004238+00
             4 |       1 | 2026-08-11 22:08:30.010029+00 | 2026-08-11 22:08:30.014802+00
             5 |       2 | 2026-08-12 15:12:16.411774+00 | 2026-08-12 15:14:27.581897+00
(5 rows)

Could close the connection as expected.

 */



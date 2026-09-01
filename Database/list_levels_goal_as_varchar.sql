/*
Lists the levels available for a new game.

Input: player's user_id.
	This requires to give the information about user_id to frontend.
Fn returns the table with:
	levels,
	information about which ones the player won.

*/

drop function if exists list_levels_goal_as_varchar(INTEGER, INTEGER, VARCHAR);


create or replace function list_levels_goal_as_varchar(
	session_id int, u_id int, session_hash VARCHAR
)
returns table (
	level_id smallint
	, description VARCHAR(100)
	, full_fen VARCHAR(100) -- full fen, including the side to play.
	, goal VARCHAR
	, won boolean
)
language plpgsql 
SECURITY definer -- makes the fn callable from the game, although it does not have the rights for individual SELECTs
as $$
DECLARE
	level_user SMALLINT;
	session_exists boolean;
BEGIN
	-- check user exists and is active
	session_exists := sess_exists(
		session_id => list_levels_goal_as_varchar.session_id,
		u_id => list_levels_goal_as_varchar.u_id,
		session_hash => list_levels_goal_as_varchar.session_hash);

	if session_exists then
		level_user := (SELECT highest_level_won
						FROM users u
						WHERE u.user_id = u_id
			);
		RETURN query
			SELECT 
				lf.level_id
				, lf.description
				, lf.full_fen
				, lf.goal::VARCHAR as goal
				, (lf.level_id <= level_user) as won
			FROM levels_fen lf;
	else
		raise exception 'Attempt to get levels without info about session.';
	end if;
end;
$$;
-- CREATE FUNCTION

-- test

select * from list_levels_goal_as_varchar(session_id => 1 , u_id => 1, session_hash => '53388382b1548a525935ff1872082f70fa5efb46ae04185f2b41edc1dc30ba0c'::VARCHAR);
/*
 level_id |           description           |            full_fen             | goal | won 
----------+---------------------------------+---------------------------------+------+-----
        1 | Barbieri-Saavedra, main line    | 8/8/1KP5/3r4/8/8/8/k7 w - - 0 1 | 1-0  | f
        2 | Pawn vs pawn near the border    | 8/8/5k2/8/p7/8/1PK5/8 w - - 0 1 | 1-0  | f
        3 | Barbieri-Saavedra, best defense | 8/8/1KP5/3r4/8/8/8/k7 w - - 0 1 | 1-0  | f
(3 rows)

*/

-- check that the column 'goal' has been converted to VARCHAR
SELECT
	level_id,
	left(ll.goal, 2) -- a nontrivial string function
FROM list_levels_goal_as_varchar(session_id => 1 , u_id => 1, session_hash => '53388382b1548a525935ff1872082f70fa5efb46ae04185f2b41edc1dc30ba0c'::VARCHAR) ll;
/*
 level_id | left 
----------+------
        1 | 1-
        2 | 1-
        3 | 1-
(3 rows)

*/

-- check the session hash is necessary
select * from list_levels_goal_as_varchar(session_id => 1 , u_id => 1, session_hash => 'ff'::VARCHAR);
-- ERROR (expected):  Attempt to get levels without info about session.


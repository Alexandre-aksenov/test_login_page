/*
Lists the levels available for a new game.

Input: player's user_id.
	This requires to give the information about user_id to frontend.
Fn returns the table with:
	levels,
	information about which ones the player won.
	
Possible improvement: check that the user exists.
*/

drop function if exists list_levels_user_id(INTEGER, INTEGER, VARCHAR);


create or replace function list_levels_user_id(
	session_id int, u_id int, session_hash VARCHAR
)
returns table (
	level_id smallint
	, description VARCHAR(100)
	, full_fen VARCHAR(100) -- full fen, including the side to play.
	, goal game_status
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
		session_id => list_levels_user_id.session_id,
		u_id => list_levels_user_id.u_id,
		session_hash => list_levels_user_id.session_hash);

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
					, lf.goal
					, (lf.level_id <= level_user) as won
				FROM levels_fen lf;
	else
		raise exception 'Attempt to get levels without info about session.';
	end if;
end;
$$;
-- CREATE FUNCTION


-- test, run as the user 'game'
select * from list_levels_user_id(session_id => 2 , u_id => 1, session_hash => '527fb4d903e7c614299f8a91eec7a02e2e306bfb54cbd399cc52d46b22bc0284'::VARCHAR );
/*
 level_id |           description           |            full_fen             | goal | won 
----------+---------------------------------+---------------------------------+------+-----
        1 | Barbieri-Saavedra, main line    | 8/8/1KP5/3r4/8/8/8/k7 w - - 0 1 | 1-0  | f
        2 | Pawn vs pawn near the border    | 8/8/5k2/8/p7/8/1PK5/8 w - - 0 1 | 1-0  | f
        3 | Barbieri-Saavedra, best defense | 8/8/1KP5/3r4/8/8/8/k7 w - - 0 1 | 1-0  | f
(3 rows)

 */

select * from list_levels_user_id(session_id => 2 ,  u_id => 1, session_hash => 'ff'::VARCHAR); -- wrong session_id
-- -> ERROR:  Attempt to get levels without info about session.


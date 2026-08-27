/*
Lists the levels available for a new game.

Input: player's user_id.
	This requires to give the information about user_id to frontend.
Fn returns the table with:
	levels,
	information about which ones the player won.
	
Possible improvement: check that the user exists.
*/

drop function if exists list_levels_user_id(INTEGER);


create or replace function list_levels_user_id(
	u_id INTEGER
)
returns table (
	level_id smallint
	,full_fen VARCHAR(100) -- full fen, including the side to play.
	, goal VARCHAR(7)
	, won bool
)
language plpgsql 
SECURITY definer -- makes the fn callable from the game, although it does not have the rights for individual SELECTs
as $$
DECLARE
	level_user SMALLINT;
BEGIN
	level_user := (SELECT highest_level_won
					FROM users u
					WHERE u.user_id = u_id
		);
	RETURN query
			SELECT 
				lf.level_id
				, lf.full_fen
				, lf.goal
				, (lf.level_id <= level_user) as won
			FROM levels_fen lf;
end;
$$;
-- CREATE FUNCTION

-- test

select * from list_levels_user_id(u_id => 3);
/*
 level_id |            full_fen             | goal | won 
----------+---------------------------------+------+-----
        1 | 8/8/1KP5/3r4/8/8/8/k7 w - - 0 1 | 1-0  | f
        2 | 8/8/5k2/8/p7/8/1PK5/8 w - - 0 1 | 1-0  | f
        3 | 8/8/1KP5/3r4/8/8/8/k7 w - - 0 1 | 1-0  | f
 */


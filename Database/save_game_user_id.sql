DROP procedure IF exists save_game_user_id;


create or replace procedure save_game_user_id(
	session_id int
	, session_hash VARCHAR
	, user_id INTEGER
	, level_id smallint
	, moves JSONB 
	, inout new_save_id int  DEFAULT 0 -- used only as the output
)
language plpgsql
SECURITY definer -- makes the proc callable from the game, although it does not have the rights for individual INSERT
AS $$
declare
	session_exists boolean;
BEGIN
	-- test whether the session exists
	session_exists := sess_exists(
		session_id => save_game_user_id.session_id,
		u_id => save_game_user_id.user_id,
		session_hash => save_game_user_id.session_hash);	
	
	IF (session_exists) THEN
		INSERT INTO saves (user_id, level_id, moves, fen, status) VALUES
		(save_game_user_id.user_id, save_game_user_id.level_id , save_game_user_id.moves, NULL, '*')
		;
		new_save_id := currval('saves_save_id_seq');
	ELSE
		raise exception 'save_game_user_id: the session does not exist';
	END IF;
END;
$$;
-- CREATE PROCEDURE


-- test 1, run as the user 'game'

call save_game_user_id(
	session_id => 2,
	user_id => 1, 
	session_hash => '527fb4d903e7c614299f8a91eec7a02e2e306bfb54cbd399cc52d46b22bc0284'::VARCHAR, 
	level_id => 1::smallint,
	moves => '[{"player": "c6c7", "opponent": "d5d6"}]'::JSONB
);
/*

 new_save_id 
-------------
           1
(1 row)
*/

-- run as admin
select * from saves;
/*

 
 save_id | user_id | level_id |                  moves                   | fen | status 
---------+---------+----------+------------------------------------------+-----+--------
       1 |       1 |        1 | [{"player": "c6c7", "opponent": "d5d6"}] |     | *
(1 row)

(user_id = 3 is the id of the user 'test_user2' at the moment of testing , OK)

 */

-- test 2 (wrong session_hash), run as the user 'game'
call save_game_user_id(
	session_id => 2,
	user_id => 1, 
	session_hash => 'ff'::VARCHAR, 
	level_id => 1::smallint,
	moves => '[{"player": "c6c7", "opponent": "d5d6"}]'::JSONB
);
-- ERROR (expected):  save_game_user_id: the session does not exist

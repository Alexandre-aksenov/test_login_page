DROP procedure IF exists save_game_user_id;


create or replace procedure save_game_user_id(
	user_id INTEGER
	, level_id smallint
	, moves JSONB 
	, inout new_save_id int  DEFAULT 0 -- used only as the output
)
language plpgsql
SECURITY definer -- makes the proc callable from the game, although it does not have the rights for individual SELECT, INSERT
AS $$
declare
	user_exists boolean; 
BEGIN
	-- test whether the user exists
	user_exists := (EXISTS (
		SElECT
			u.user_id
		FROM users u
		where u.user_id = save_game_user_id.user_id
	));
	
	IF (user_exists) THEN
		INSERT INTO saves (user_id, level_id, moves, fen, status) VALUES
		(save_game_user_id.user_id, save_game_user_id.level_id , save_game_user_id.moves, NULL, '*')
		;
		new_save_id := currval('saves_save_id_seq');
	ELSE
		raise exception 'save_game_user_id: the user with id % does not exist', save_game_user_id.user_id;
	END IF;
END;
$$;
-- CREATE PROCEDURE

-- test

call save_game_user_id(user_id => 3, level_id => 1::smallint, moves => '[{"player": "c6c7", "opponent": "d5d6"}]'::JSONB );
/*

 new_save_id 
-------------
           4
(1 row)
*/


select * from saves;
/*
 save_id | user_id | level_id |                  moves                   | fen | status 
---------+---------+----------+------------------------------------------+-----+--------
       1 |       3 |        1 | [{"player": "c6c7", "opponent": "d5d6"}] |     | *
       2 |       3 |        1 | [{"player": "c6c7", "opponent": "d5d6"}] |     | *
       3 |       3 |        1 | [{"player": "c6c7", "opponent": "d5d6"}] |     | *
       4 |       3 |        1 | [{"player": "c6c7", "opponent": "d5d6"}] |     | *
(4 rows)

(user_id = 3 is the id of the user 'test_user2' at the moment of testing , OK)

 */

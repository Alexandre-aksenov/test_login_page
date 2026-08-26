-- Creation of tables.

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    login VARCHAR(255), -- UNIQUE can be added in next ver
    pwd VARCHAR(255), -- pwd can be in clear (for testing from PSQL) or hashed on frontend
    registration_date DATE,
    highest_level_won smallint
);
-- Replace by scram-sha-256 in future,
-- see: https://www.postgresql.org/docs/current/auth-password.html

-- CREATE TABLE


CREATE TABLE connections (
	connection_id SERIAL PRIMARY KEY,
	user_id INT REFERENCES users(user_id),
	start_session timestamptz NOT NULL, 
	end_session timestamptz
);
-- CREATE TABLE


CREATE TABLE levels_fen (
	level_id smallint PRIMARY KEY,
	-- TOADD in next version: description
	full_fen VARCHAR(100), -- full fen, including the side to play.
	goal VARCHAR(7) -- another possibility: OPTION('*', '1-0', '1/2-1/2', '0-1')
);
-- CREATE TABLE


-- Opponent's moves of the main line. To be used in the full game.
-- The opponent should play them if the game reaches the corressponding position.
CREATE TABLE levels_main_line (
	move_id SERIAL PRIMARY key,
	level_id smallint REFERENCES levels_fen(level_id),
	fen VARCHAR(100),
	forced_move VARCHAR(5)
);
-- CREATE TABLE


CREATE TABLE saves (
	save_id SERIAL PRIMARY key,
    user_id INT REFERENCES users(user_id),
    level_id smallint REFERENCES levels_fen(level_id),
    moves JSONB,
    fen VARCHAR(100),
    status VARCHAR(7) -- another possibility: OPTION('*', '1-0', '1/2-1/2', '0-1')
);
-- a column 'parent_save' can be added
-- CREATE TABLE


-- Create Default user

INSERT INTO users (login, pwd, registration_date, highest_level_won) VALUES
('default_user', 'default_pwd', '2026-08-11', 0)
;

-- Create 3 levels for the free version

INSERT INTO levels_fen (level_id, full_fen, goal) values 
(1, '8/8/1KP5/3r4/8/8/8/k7 w - - 0 1', '1-0'), -- pos in the frontend-only version
(2, '8/8/5k2/8/p7/8/1PK5/8 w - - 0 1', '1-0'), -- W: Kc2, b2. B: Kf6, a4
(3, '8/8/1KP5/3r4/8/8/8/k7 w - - 0 1', '1-0') -- 1st position, but much harder
;
-- INSERT 0 3

INSERT INTO levels_main_line (level_id, fen, forced_move) values
(1, '8/2P5/8/8/8/3r4/2K5/k7', 'd3d4'),
(1, '8/2P5/8/8/8/8/2K5/k2r4', 'd1d4'),
(1, '8/2P5/8/3r4/1K6/8/8/k7', 'd5d4')
;
-- INSERT 0 3

-- Structure of tables for 'test_login_page'.


CREATE TYPE game_status AS ENUM ('*', '1-0', '1/2-1/2', '0-1');
-- CREATE TYPE

-- Creation of tables.

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    login VARCHAR(255) unique NOT NULL, -- added UNIQUE
    pwd VARCHAR(64) NOT NULL, -- A more restrictive option: bytea NOT NULL; pwd can be in clear (for testing from PSQL) or hashed on frontend
    registration_date DATE NOT NULL,
    highest_level_won smallint NOT null check (highest_level_won >= 0)
);
-- Replace the login procedure by scram-sha-256 in future,
-- see: https://www.postgresql.org/docs/current/auth-password.html

-- CREATE TABLE


CREATE TABLE connections (
	connection_id SERIAL PRIMARY KEY,
	user_id INT REFERENCES users(user_id) NOT NULL,
	session_hash_key VARCHAR(64) NOT NULL, -- A more restrictive option: bytea  NOT NULL
	start_session timestamptz NOT NULL, 
	end_session timestamptz -- NULL if the session is ongoing
);
-- CREATE TABLE


CREATE TABLE levels_fen (
	level_id smallint PRIMARY KEY,
	description VARCHAR(100) NOT NULL, -- added
	full_fen VARCHAR(100) NOT NULL, -- full fen, including the side to play.
	goal game_status NOT NULL CHECK (goal != '*')
);
-- CREATE TABLE


-- Opponent's moves of the main line.
-- The opponent should play them if the game reaches the corresponding position
CREATE TABLE levels_main_line (
	move_id SERIAL PRIMARY key,
	level_id smallint REFERENCES levels_fen(level_id) NOT NULL,
	fen VARCHAR(100) NOT NULL,
	forced_move VARCHAR(5) NOT NULL
);
-- CREATE TABLE 


CREATE TABLE saves (
	save_id SERIAL PRIMARY key,
    user_id INT REFERENCES users(user_id) NOT NULL,
    level_id smallint REFERENCES levels_fen(level_id) NOT NULL,
    moves JSONB NOT NULL,
    fen VARCHAR(100), -- Position after the moves. NULL in the mini-game, but will be computed and saved in DB in the full game
    status game_status NOT NULL -- Status after the moves. '*' in the mini-game, will be computed and filled in the full game
);
-- parent_save can be added as column
-- CREATE TABLE 


-- Create Default user

INSERT INTO users (login, pwd, registration_date, highest_level_won) VALUES
('default_user', 'deadbeef', '2026-08-30', 0);
-- 'deadbeef' is composed of hexadecimal digits
-- INSERT 0 1

-- Create 3 levels for the free version

INSERT INTO levels_fen (level_id, description, full_fen, goal) values 
(1, 'Barbieri-Saavedra, main line', '8/8/1KP5/3r4/8/8/8/k7 w - - 0 1', '1-0'), -- pos in the frontend-only version
(2, 'Pawn vs pawn near the border', '8/8/5k2/8/p7/8/1PK5/8 w - - 0 1', '1-0'), -- W: Kc2, b2. B: Kf6, a4
(3, 'Barbieri-Saavedra, best defense', '8/8/1KP5/3r4/8/8/8/k7 w - - 0 1', '1-0') -- 1st position, but much harder
;
-- INSERT 0 3

INSERT INTO levels_main_line (level_id, fen, forced_move) values
(1, '8/2P5/8/8/8/3r4/2K5/k7', 'd3d4'),
(1, '8/2P5/8/8/8/8/2K5/k2r4', 'd1d4'),
(1, '8/2P5/8/3r4/1K6/8/8/k7', 'd5d4')
;
-- INSERT 0 3


-- Add a  user 'game' for connection to DB from the game middleware.
-- Give it priviledges to run functins and procedures.

-- DROP ROLE IF EXISTS role_game;

CREATE ROLE game_role;
-- CREATE ROLE

GRANT execute
ON PROCEDURE signup, sign_in_user_id, logout, save_game_user_id
to game_role;
-- GRANT

GRANT execute
ON FUNCTION list_levels_user_id
to game_role;
-- GRANT

-- create user.
-- https://www.postgresql.org/docs/current/sql-createuser.html
CREATE USER game
WITH PASSWORD 'pwd_game'
ROLE game_role;
-- CREATE ROLE
-- psql answer is the same, which corresponds to this:
-- "CREATE USER is now an alias for CREATE ROLE. "
-- (https://www.postgresql.org/docs/current/sql-createuser.html)

-- Connect:
/*
From Terminal:
psql -h localhost -p 5433 -U game -d mydatabase -W

From local middleware:
let res_client = tokio_postgres::connect("host=localhost port=5433 user=game password=pwd_game dbname=mydatabase", NoTls).await;

 */


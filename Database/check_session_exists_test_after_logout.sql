-- test, call as the admin after logging out
SELECT sess_exists(session_id => 1, u_id => 1, session_hash => '53388382b1548a525935ff1872082f70fa5efb46ae04185f2b41edc1dc30ba0c'::VARCHAR);
/*
sess_exists 
-------------
 f
(1 row)
*/
-- False, as expected

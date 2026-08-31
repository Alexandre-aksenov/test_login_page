-- test, call as the admin after logging out
SELECT sess_exists(session_id => 2, u_id => 1, session_hash => '527fb4d903e7c614299f8a91eec7a02e2e306bfb54cbd399cc52d46b22bc0284'::VARCHAR);
/*
sess_exists 
-------------
 f
(1 row)
*/
-- False, as expected

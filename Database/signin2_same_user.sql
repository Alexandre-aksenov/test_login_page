-- Sign-in as the test user for a 2nd time. A new hash is produced.

call sign_in_user_id(login => 'default_user', pwd => 'deadbeef');
/*

 new_connection_id | user_id |                         new_session_hash                         
-------------------+---------+------------------------------------------------------------------
                 2 |       1 | b17a17dd2ef630b5f3ef8bb44ad7837396852bd750e8ce556896f30861791f75
(1 row)


*/


-- Logout
call logout(session_id => 2,
	user_id => 1, 
	session_hash => 'b17a17dd2ef630b5f3ef8bb44ad7837396852bd750e8ce556896f30861791f75'::VARCHAR);

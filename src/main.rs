use dioxus::prelude::*;
use dioxus_sdk_storage::use_persistent; // for persistent storage of login info

use sha3_rust::sha3_256; // for hashing the pwd in the DX client (frontend)
use hex;

use itertools::join;

use test_login_page::{solution_lvl1, to_json, // for the minigame.
                      decode_session_hash, // for decoding session hash received from the server in middleware fn 'login_user_connection_id'.
                      ConnectionInfo}; // for gathering connection info into 1 structure

#[cfg(feature = "server")]
use axum::extract::Query;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct LevelInfo{
    level_id: i16,
    full_fen: String,
    goal: String,
    won: bool,
}

/// Concatenates levels, surrounding each of them with an HTML paragraph tag
fn display_concat_levels_par(lvls: &[LevelInfo]) -> String {
    join(lvls.iter().map(|lvl| format!("<p>{:?}</p>", lvl)), "\n")
}

/// The next level to play, if any.
fn next_unsolved_level(lvls: &[LevelInfo]) -> Option<i16> {
    lvls
        .iter()
        .find(|lvl| !lvl.won)
        .map(|lvl| lvl.level_id)
}


/// Describes the list of pages
/// https://dioxuslabs.com/learn/0.7/essentials/router/#creating-a-routable-enum
#[derive(Routable, Clone, PartialEq)] // manual suggests adding the Debug trait
enum Route {
    #[route("/")]
    Home {},
    #[route("/trial")]
    Trial {},
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Signup {},
}

/// Main fn for the frontend.
fn main() {
    dioxus::launch(App);
}

/// Styling.
#[component]
fn App() -> Element {
    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Page for the trial game.
#[component]
fn Trial() -> Element {
    rsx! {
        div { "The original (frontend) game will go here." }
    }
}

/// Signup page.
#[component]
fn Signup() -> Element {
    let mut candidate_user_login = use_signal(|| String::new());
    let mut cleartext_pwd = use_signal(|| String::new());

    let mut signed_up = use_signal(|| false);
    let mut message = use_signal(|| String::from("Sign up as a new user"));
    let mut message_end_page = use_signal(|| String::new());

    message_end_page.set(
        format!("Len of current Login and password: {}, {}",
                candidate_user_login().chars().count(),
                cleartext_pwd().chars().count()));

    rsx! {
        div {
            class : "msg",
            "{message}"
        }

        if signed_up() == false {
            div {
                class: "row col2",
                div {
                    input {
                        type : "text",
                        placeholder: "Login",
                        oninput: move |event| {
                            candidate_user_login.set(event.value());
                        },
                    }
                }
                div {
                    input {
                        type : "text", // will be "password" in the deployed version
                        placeholder: "Password",
                        oninput: move |event| {
                            cleartext_pwd.set(event.value());
                        },
                    }
                }
            },

            // Button "Sign up"
            button {
                class : "btn-primary",
                onclick: move |_|  async move {
                    // check whether nontrivial login, password have been supplied
                    if (candidate_user_login().chars().count() > 3) && (cleartext_pwd().chars().count() > 3)
                        {
                            message.set(String::from("Signing up..."));
                            // compute password hash
                            let pwd_hash_keccak256 = sha3_256(cleartext_pwd().as_bytes()); // [u8; 32]
                            let hash_str = hex::encode(pwd_hash_keccak256); // String of hex digits

                            message.set(String::from("Password hash computed"));
                            // call middleware fn
                            let res_signup = register(candidate_user_login(), hash_str).await;

                            match res_signup {
                                Ok(signup_msg) => {
                                    signed_up.set(true);
                                    message.set(signup_msg);
                                },
                                Err(err_msg) => {message.set(format!("Could not reach middleware: {}", err_msg));},
                            }
                        } else {
                        message.set(format!("Login and password should be of >= 4 symbols. Received: {}, {}", candidate_user_login().chars().count(), cleartext_pwd().chars().count()));
                    } // end of check for a nontrivial login, pwd

                }, // end of onclick action
                "Sign up"
            } // end of button

        } // end of if signed_up() == false

        div {
            class : "msg",
            "{message_end_page}"
        }
    } // end of rsx!
}


/// Login / logout page, + signed-in game (TOADD)
#[component]
fn Login() -> Element {
    // Signal variables (visible in the window)
    let mut signed_in = use_persistent("signed_in", || false);

    let mut user_login = use_persistent("user_login", || String::new());
    let mut pagewize_conn_id: Signal<i32> = use_persistent("connection_id", || 0);
    let mut user_id = use_persistent("user_id", || 0);
    //  0 stands for no active connection.

    // session_hash. Its initial value is None: Signal(Option<[u8; 32]>) ), standing for no active session.
    let mut session_hash: Signal<Option<[u8; 32]>> = use_persistent("session_hash", || None );

    // new in branch 'struct_connection_info'.
    let mut connection_info: Signal<Option<ConnectionInfo>> = use_persistent("connection_info", || None);
    // None means: not connected.

    // Useful during login
    let mut candidate_user_login = use_signal(|| String::new());
    let mut cleartext_pwd = use_signal(|| String::new());

    let mut response_msg: Signal<String> = use_signal(|| String::new());
    let mut button_text = use_signal(|| String::new());




    // For the signed-in game
    let mut user_list_levels: Signal<Vec<LevelInfo>> = use_signal(|| Vec::new());
    let mut str_levels: Signal<String> = use_signal(|| String::from("DB response will go here "));
    let mut next_level: Signal<Option<i16>> = use_signal(|| None);
    let mut current_level: Signal<Option<i16>> = use_signal(|| None);

    // new in commit of 25/8/2026, 18:40
    let mut num_correct_moves: Signal<u16> = use_signal(|| 0);

    let mut player_move1: Signal<String> = use_signal(|| String::from(""));
    let mut player_move2: Signal<String> = use_signal(|| String::from(""));
    let mut player_move3: Signal<String> = use_signal(|| String::from(""));

    let mut last_saveid: Signal<i32> = use_signal(|| 0);
    let correct_sol = use_signal(|| solution_lvl1());

    // initialization depending on whether the user is signed in
    if *signed_in.read() {
        button_text.set(String::from("Sign out"));
        response_msg.set(format!("You are signed in as: {}", *user_login.read()));
    } else { // This screen appears on the 1st load
        button_text.set(String::from("Sign in"));
        response_msg.set(String::from("Please, sign in"));
    }

    rsx! {
        fieldset { // gray rectangle. Its effect is specified in CSS.
            legend { "Account" }

            div {
                class : "msg",
                "{response_msg}"
            }

            if connection_info().is_none() { // *signed_in.read() == false
                div {
                    class: "row col2",
                    div {
                        input {
                            type : "text",
                            placeholder: "Login",
                            oninput: move |event| {
                                candidate_user_login.set(event.value());
                            },
                        }
                    }
                    div {
                        input {
                            type : "text", // will be "password" in the deployed version
                            placeholder: "Password",
                            oninput: move |event| {
                                cleartext_pwd.set(event.value());
                            },
                        }
                    }
                },
            }

            // Button "Sign in"/"Sign out" described by the variable 'button_text'.
            // Calls the server fn.
            div {
                button {
                    class : "btn-primary",
                    onclick : move |_| async move {
                        match &mut (connection_info()) {
                            &mut None => {
                                // Sign-in user
                                // Calls the middleware fn 'login_user_connection_id'

                                let hashed_pwd = hex::encode(sha3_256(cleartext_pwd().as_bytes()));

                                let res_response = login_user_connection_id(candidate_user_login(), hashed_pwd).await;

                                let response = res_response.unwrap_or((0, -4, None)); // -4 means: DX server did not reply

                                let response_text = match (response.1 > 0) {
                                    true => {// In case of success, update state variables:
                                        *pagewize_conn_id.write() = response.1;
                                        *signed_in.write() = true; // The button changes to "Sign out"
                                        *user_login.write() = candidate_user_login.read().clone();
                                        *user_id.write() = response.0;
                                        *session_hash.write() = response.2;

                                        // -> structure
                                        *connection_info.write() = Some(ConnectionInfo {
                                                connection_id: response.1,
                                                user_id: response.0,
                                                session_hash: response.2.unwrap(),
                                            });

                                        // clear candidate login (local var to this session).
                                        candidate_user_login.set(String::new());

                                        format!("Login successful, connection id {}", response.1)
                                    },
                                    false => format!("Login failed, code {}", response.1),
                                };

                                response_msg.set(response_text);
                            }, // end of None branch
                            &mut Some(connection) => { // *signed_in.read() == true
                                // Sign out user
                                // Calls the server fn 'logout'.

                                let res_logout = logout(
                                            connection.connection_id,
                                            connection.user_id,
                                            hex::encode(&connection.session_hash))
                                        .await;

                                match res_logout {
                                    Ok(msg_logout) => {
                                        // modify state variables.
                                        response_msg.set(format!("{}", msg_logout));
                                        *signed_in.write() = false;

                                        user_login.set(String::new());
                                        cleartext_pwd.set(String::new());

                                        *pagewize_conn_id.write() = 0;
                                        *user_id.write() = 0;

                                        *session_hash.write() = None;

                                        *connection_info.write() = None;
                                    },
                                    Err(e) => {
                                        response_msg.set(format!("Tried to logout from session {}. Error: {}", connection.connection_id, e));
                                    }
                                }  // end of 'match res_logout'
                            } // end of "if *signed_in.read() == true" -> end of Some branch
                        } // end of match
                    }, // end of 'onclick'

                    {button_text()}
                } // end of 'button'
            } // end of div
        } // end of fieldset


        // USEFUL content: the mini-game
        match *connection_info.read() {
            None => rsx! {
                div {"Logged-in game will appear after logging in."}
            },
            Some(connection) => rsx! {
                // div {"Connected as id {inner.user_id}."}
                div {"Logged-in game coming soon."}

                button { // "New game"
                    class : "btn-primary",
                    onclick : move |_| async move {
                        str_levels.set(String::from("Calling the list_levels() fn"));

                        // query the list of levels for the user
                        user_list_levels.set(
                            list_levels_uid(connection.connection_id,
                                            connection.user_id,
                                            hex::encode(&connection.session_hash))
                            .await
                            .unwrap_or(Vec::new())
                        );

                        str_levels.set(display_concat_levels_par(&*user_list_levels.read()));
                        next_level.set(next_unsolved_level(&*user_list_levels.read()));
                    }, // end of onclick action.
                    "New game"
                } // end of button

                div {dangerous_inner_html: "{str_levels.read()}"}

                // The mini-game itself
                if (next_level.read().is_none()) { // -> 2nd pattern-matching ()
                    div {"No more levels (you've solved the game or should click New Game to see options)"}
                }
                else {
                    div {"Can start level {next_level.read().unwrap()}"}

                    button { // "Start next level"
                        class : "btn-primary",
                        onclick : move |_| async move {
                            str_levels.set(String::from("Starting the level..."));

                            current_level.set(*next_level.read());
                        }, // end of onclick action.
                        "Start next level"
                    } // end of button

                    // The mini-game itself
                    if (current_level.read().is_none()) { // -> 3rd pattern-matching ()
                        div {"The mini-game will appear here"}
                    }
                    else {
                        div {"Started level {current_level.read().unwrap()}"}
                        // TOADD the mini-game here

                        div { // input 1st player's move
                            input {
                                type : "text",
                                placeholder: "1st move",
                                oninput: move |event| {
                                    player_move1.set(event.value());
                                },
                            }
                        } // end of the div to input text field

                        if *player_move1.read() ==
                            (correct_sol.read())[0].player.to_string() {
                            // Update num_correct_moves

                            button { // "Correct!"
                                class : "btn-primary",
                                onclick : move |_| async move {
                                    *num_correct_moves.write() = 1;
                                }, // end of onclick action.
                                "Correct!"
                            } // end of button
                        }
                        else {
                            div {"Incorrect."}
                        } // end of if/else block: reaction to user's 1st move

                        // Show opponent's reply
                        if *num_correct_moves.read() >= 1 {
                                div {"Opponent's reply: {(correct_sol.read())[0].opponent.to_string()}"}
                        }

                        // 2nd move will be shown after the first move is correct.

                        // button save progress
                        if *num_correct_moves.read() >= 1 {
                            button { // "Save"
                                class : "btn-primary",
                                // onclick
                                onclick : move |_| async move {

                                    let res_last_saveid = save_game_uid(
                                        connection.connection_id,
                                        connection.user_id,
                                        hex::encode(&connection.session_hash),
                                        current_level.read().expect("level could not be read"),
                                        to_json(&(correct_sol.read())[..(*num_correct_moves.read() as usize)]))
                                    .await;

                                    // save_id for printing below
                                    match res_last_saveid {
                                        Ok(saveid) => {
                                            *last_saveid.write() = saveid;
                                        },
                                        Err(_) => {
                                            *last_saveid.write() = -1;
                                        }
                                    }
                                }, // end of onclick action.
                                "Save progress"
                            } // end of button to save progress

                        } // end of: if num_correct_moves.read() >= 1

                    } // end of: if current_level is not none

                } // end of: if next_level is not none

            } // end of hand connection_info => Some(connection:ConnectionInfo)
        } // end of match *connection_info.read()

    } // end of rsx!
} // end of component


#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" } // Dioxus: interfaces that run anywhere
            div { id: "links",
                a { href: "/trial", "📚 Trial without account" }
                a { href: "/login", "🚀 Login" }
                a { href: "/signup", "📡 Sign up" }
            }
        }
    }
}



// Middleware-side code. 
/// Register a new user in DB. The password has already been hashed in the browser.
/// Calls the DB procedure 'signup'.
/// Adds a row to the table 'users'.
#[server()]
async fn register(
    login: String,
    hash_pwd: String
) -> Result<String, ServerFnError>
{
    // import
    use tokio_postgres::NoTls;

    // establish a connection
    let res_client = tokio_postgres::connect("host=localhost port=5433 user=game password=pwd_game dbname=mydatabase", NoTls).await;

    let reply = match res_client { // Result<String, ServerFnError>
        Ok((mut client, connection)) => {
            // Spawn 'connection'.
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            // Query
            let res_signup  = client.execute(
                "call signup(login => $1::VARCHAR, pwd => $2::VARCHAR)", // statement
                &[&login, &hash_pwd]) // params
                .await; // Result<u64, Error>

            // Process the result
            match res_signup {
                Ok(_) => Ok("Signed up successfully".to_string()),
                Err(e) => Err(ServerFnError::Request(dioxus_fullstack::RequestError::Body(format!("Signup failed: {}", e))))
            }
        },
        Err(e) => { Err(ServerFnError::Request(dioxus_fullstack::RequestError::Request(format!("Failed to connect to database while attempting to sign up: {}", e)))) }
    };

    match reply {
        Ok(reply) => Ok(reply),
        Err(e) => Err(e),
    }
}


/// Login fn. Calls the DB procedure 'sign_in_user_id'.
/// Returns:
/// new_user_id > 0 if connection to DB is successful.
///                   = 0 otherwize
/// new connection_id > 0 if connection to DB is successful.
///  -1 should be returned if error on connection to DB.
///  -2 should be returned if the row does not contain "new_connection_id"
///    (should not happen, according to the signature of the DB procedure).
///  -3 if login is rejected by DB (because the user does not exist or the password is wrong).
/// new_session_hash: Option<[u8; 32]>,
///   Some([u8; 32]) stands for success (the hash is then received from the DB),
///   None stands for error.
/// If login is successful, adds a row to the table 'connections'.
#[server()]
async fn login_user_connection_id(
    user_login: String, // if &str -> error[E0521]: borrowed data escapes outside of function
    hash_pwd: String
) -> Result<(i32, i32, Option<[u8; 32]>), ServerFnError> // <- dioxus::prelude::ServerFnError
{
    use tokio_postgres::NoTls;

    let res_client = tokio_postgres::connect("host=localhost port=5433 user=game password=pwd_game dbname=mydatabase", NoTls).await;

    let (user_id, connection_id, session_hash) = match res_client {
        Ok((mut client, connection)) => {

            // Spawn 'connection'.
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            // Query
            let res_login = client.query_one(
                "call sign_in_user_id(login => $1::VARCHAR, pwd => $2::VARCHAR);", // statement
                &[&user_login, &hash_pwd]) // params
                .await; // Result<Row, Error>


            let mut new_connection_id: i32 = 0;
            let mut new_user_id: i32 = 0;
            let mut new_session_hash: Option<[u8; 32]> = None;

            match res_login {
                Ok(row) => {
                    new_connection_id = row.try_get("new_connection_id").unwrap_or(-2);
                    new_user_id = row.try_get("user_id").unwrap_or(0);
                    // new_session_hash = row.try_get("session_hash").unwrap_or(None);
                    new_session_hash = match row.try_get("new_session_hash") {
                        // decoding: DB returns hex encoding (VARCHAR of len 64)
                        Ok(hash_varchar) => decode_session_hash(hash_varchar), // hash_varchar: VARCHAR in Postgres, attempt to read as &str
                        Err(_) => None,
                    };
                }
                Err(e) => {
                    eprintln!("Login failed : {}", e);
                    new_connection_id = -3;
                }
            }

            (new_user_id, new_connection_id, new_session_hash)
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            (0, -1, None)
        }};

    Ok((user_id, connection_id, session_hash))
}


/// Log out.  Calls the DB procedure 'logout'.
/// Input:
///     connection_id (i32),
///     user_id (i32),
///     session_hash_str (String) // decoded from Option([u8; 32]) on the frontend
/// received from the login function
#[server()]
async fn logout(
    connection_id: i32,
    user_id: i32,
    session_hash_str: String
) -> Result<String, ServerFnError>
{
    use tokio_postgres::NoTls;

    let res_client = tokio_postgres::connect("host=localhost port=5433 user=game password=pwd_game dbname=mydatabase", NoTls).await;

    let reply = match res_client { // Result<String, ServerFnError> to send to the client
        Ok((mut client, connection)) => {
            // Logout call itself

            // Spawn 'connection'.
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error during logout: {}", e);
                }
            });

            let res_logout = client.execute(
                "call logout(session_id => $1, user_id => $2, session_hash => $3::VARCHAR);",
                &[&connection_id, &user_id, &session_hash_str])
                .await; // Result<u64, Error>

            match res_logout {
                Ok(_) => Ok(String::from("Logged out successfully.")),
                Err(e) => Err(ServerFnError::Request(dioxus_fullstack::RequestError::Body(format!("Failed to sign_out: {}", e)))),
            }

        },
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);

            Err(ServerFnError::Request(dioxus_fullstack::RequestError::Request(format!("Failed to connect to database while attempting to sign_out: {}", e))))
        }
    }; // end: let reply =

    match reply {
        Ok(reply) => Ok(reply),
        Err(e) => Err(e),
    }
}


/// Read the level list from DB. Calls the DB function 'list_levels_uid'.
/// Input:
///     connection_id (i32),
///     user_id (i32),
///     session_hash_str (String) // decoded from Option([u8; 32]) on the frontend
/// received from the login function.
/// Returns: vector of structs with info about each level (LevelInfo).
#[server()]
async fn list_levels_uid(
    connection_id: i32,
    user_id: i32,
    session_hash_str: String
) -> Result<Vec<LevelInfo>, ServerFnError>
{
    #[cfg(feature = "server")]
    {
        use tokio_postgres::NoTls;

        // deserialization. tokio_postgres has been imported above
        fn deserialize_row_to_level(row: &tokio_postgres::Row) -> LevelInfo {
            // crate::LevelInfo {
            LevelInfo {
                level_id: row.get("level_id"), // i16
                full_fen: row.get("full_fen"), // String
                goal: row.get("goal"), // String
                won: row.get("won"), // bool
            }
        }

        let res_client = tokio_postgres::connect("host=localhost port=5433 user=game password=pwd_game dbname=mydatabase", NoTls).await;

        let reply = match res_client {
            Ok((mut client, connection)) => {

                // Spawn 'connection'.
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });


                let res_table_lvls = client.query(
                    "select * from list_levels_goal_as_varchar(session_id => $1, u_id => $2, session_hash => $3::VARCHAR);", // statement
                    &[&connection_id, &user_id, &session_hash_str]) // params
                .await; // Result<Vec<Row>, Error>


                // process the answer of DB
                match res_table_lvls {
                    Ok(vec_row_levels) => {

                        let vec_lvls:Vec<LevelInfo> = vec_row_levels
                            .iter()
                            .map(|row| deserialize_row_to_level(row))
                            .collect();

                        Ok(vec_lvls)
                    },
                    Err(e) => { Err(ServerFnError::Request(
                        dioxus_fullstack::RequestError::Body(format!("Failed to get list of levels: {}", e))
                    ))
                    },
                }
            }, // end of OK block if the client could connect to Database
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);

                Err(ServerFnError::Request(
                    dioxus_fullstack::RequestError::Request(
                        format!("Failed to connect to database while attempting to get list of levels: {}", e)
                    )
                ))
            }
        };

        match reply {
            Ok(reply) => Ok(reply),
            Err(e) => Err(e),
        }
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server logic not available on client"))
    }
}


/// Save the game.  Calls the DB procedure 'save_game_user_id'.
/// Input:
///     connection_id (i32),
///     user_id (i32),
///     session_hash_str (String) // decoded from Option([u8; 32]) on the frontend
///     level_id,
///     moves (JSON String about player's moves and replies).
///         Currently (in the mini-game), only correct moves can be saved.
///         This represents some extra information, which will be used in the full game
///         (the player will be able to save his progress whether he is on the right path or not).
/// Returns: save_id (to use for loading).
/// Adds a row to the table 'saves'.
#[server()]
async fn save_game_uid(
    connection_id: i32,
    user_id: i32,
    session_hash_str: String,
    level_id : i16,
    moves : String) -> Result<i32, ServerFnError>
{
    #[cfg(feature = "server")]
    {
        use tokio_postgres::NoTls;

        let res_client = tokio_postgres::connect("host=localhost port=5433 user=game password=pwd_game dbname=mydatabase", NoTls).await;

        match res_client {
            Ok((mut client, connection)) => {
                //      spawn connection
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });

                //      query
                let res_save = client.query_one(
                    "call save_game_user_id(session_id => $1, user_id => $2::INTEGER, session_hash => $3::VARCHAR,  level_id => $4::smallint, moves => $5::TEXT::JSONB);", // statement
                    &[&connection_id, &user_id, &session_hash_str, &level_id, &moves]
                    )
                .await;


                //      process answer
                let mut new_save_id = 0;

                match res_save {
                    Ok(row) => { new_save_id = row.try_get(0).unwrap_or(-2); },
                    Err(e) => { eprintln!("Failed to call save_game procedure: {}", e);
                        new_save_id = -3;},
                }


                // default return value
                Ok(new_save_id)
            },
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                Err(ServerFnError::Request(
                    dioxus_fullstack::RequestError::Request(
                        format!("Failed to connect to database while attempting to save game: {}", e)
                    )
                ))
            }
        }
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server logic not available on client"))
    }
}

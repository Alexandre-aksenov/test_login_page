use dioxus::prelude::*;
use dioxus_sdk_storage::use_persistent; // for persistent storage of login info

use sha3_rust::sha3_256; // for hashing the pwd in the DX client (frontend)
use hex;

// use dioxus_shareables::{shareable, List, ListEntry};
// TOADD: backend, which communicates with PostgreSQL

#[cfg(feature = "server")]
use axum::extract::Query;



const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// shareable!(Numbers: List<usize> = [3, 5, 7].into_iter().collect());


// https://dioxuslabs.com/learn/0.7/essentials/router/#creating-a-routable-enum
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
    #[route("/signed_in_game")]
    Game {},
}

/// Main fn for the frontend.
/// -> status 404 in browser
// #[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Trial() -> Element {
    rsx! {
        div { "The original (frontend) game will go here." }
    }
}

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


/// Login / logout page
#[component]
fn Login() -> Element {
    // Signal variables (visible in the window)
    let mut signed_in = use_persistent("signed_in", || false);

    let mut user_login = use_persistent("user_login", || String::new());
    let mut pagewize_conn_id: Signal<i32> = use_persistent("connection_id", || 0);

    let mut candidate_user_login = use_signal(|| String::new());
    let mut cleartext_pwd = use_signal(|| String::new());

    let mut response_msg: Signal<String> = use_signal(|| String::new());
    let mut button_text = use_signal(|| String::new());

    // initialization depending on whether the user is signed in
    if *signed_in.read() {
        button_text.set(String::from("Sign out"));
        response_msg.set(format!("You are signed in as: {}", *user_login.read()));
    } else { // This screen appears on the 1st load
        button_text.set(String::from("Sign in"));
        response_msg.set(String::from("Please, sign in"));
    }

    rsx! {
            div {
                class : "msg",
                "{response_msg}"
            }

        if *signed_in.read() == false {
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
        // this button can look more like a button after changes in CSS. TOADAPT
        // Calls the server fn.
        div {
            button {
                class : "btn-primary",
                onclick : move |_| async move {
                    if *signed_in.read() == false {
                        // Sign-in user
                        // Calls the middleware fn 'login'

                        let hashed_pwd = hex::encode(sha3_256(cleartext_pwd().as_bytes()));

                        let res_response = login(candidate_user_login(), hashed_pwd).await;

                        let response = res_response.unwrap_or(-4); // -4 means: DX server did not reply

                        let response_text = match (response > 0) {
                            true => {// In case of success, update state variables:
                                *pagewize_conn_id.write() = response;
                                *signed_in.write() = true; // The button changes to "Sign out"
                                *user_login.write() = candidate_user_login.read().clone();

                                // clear candidate login (local var to this session).
                                candidate_user_login.set(String::new());

                                format!("Login successful, connection id {}", response)
                            },
                            false => format!("Login failed, code {}", response),
                        };

                        response_msg.set(response_text);

                    } else { // *signed_in.read() == true
                        // Sign out user
                        // Calls the server fn 'logout'.

                        let res_logout = logout(*pagewize_conn_id.read()).await;

                        match res_logout {
                            Ok(msg_logout) => {
                                // modify state variables.
                                response_msg.set(format!("{}", msg_logout));
                                *signed_in.write() = false;

                                user_login.set(String::new());
                                cleartext_pwd.set(String::new());

                                *pagewize_conn_id.write() = 0;
                            },
                            Err(e) => {
                                response_msg.set(format!("Tried to logout from session {}. Error: {}", *pagewize_conn_id.read(), e));
                            }
                        }
                    } // end of "if *signed_in.read() ..."
                }, // end of 'onclick'
                {button_text()}
            } // end of 'button'
        } // end of div
    } // end of rsx!
} // end of component


#[component]
fn Game() -> Element {
    rsx! {
        div { "logged-in game" }
    }
}


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
#[server()]
async fn register(
    login: String,
    hash_pwd: String
) -> Result<String, ServerFnError>
{
    // import
    use tokio_postgres::NoTls;

    // establish a connection
    let res_client = tokio_postgres::connect("host=localhost port=5433 user=alex password=pwd dbname=mydatabase", NoTls).await;

    let reply = match res_client { // Result<String, ServerFnError>
        Ok((mut client, connection)) => {
            // Spawn 'connection'.
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            // Query
            let query = format!("call signup(login => '{}', pwd => '{}')", login, hash_pwd);
            let res_signup = client.execute(&query, &[]).await; // -> Should be: Result<u64, Error>

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


/// Returns:
/// new connection_id > 0 if connection to DB is successful.
///  -1 should be returned if error on connection to DB.
///  -2 should be returned if the row does not contain "new_connection_id"
///    (should not happen according to the signature of the DB procedure).
///  -3 if login is rejected by DB (because the user does not exist or the password is wrong).
#[server()]
async fn login(
    user_login: String, // if &str -> error[E0521]: borrowed data escapes outside of function
    hash_pwd: String
) -> Result<i32, ServerFnError> // <- dioxus::prelude::ServerFnError
{
    use tokio_postgres::NoTls;

    let res_client = tokio_postgres::connect("host=localhost port=5433 user=alex password=pwd dbname=mydatabase", NoTls).await;

    let connection_id = match res_client {
        Ok((mut client, connection)) => {

            // Spawn 'connection'.
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });


            let query_login = format!("call sign_in(login => '{}', pwd => '{}');", &user_login, &hash_pwd);
            let res_login = client.query_one(&query_login, &[]).await;

            let mut new_connection_id: i32 = 0;

            match res_login {
                Ok(row) => {
                    new_connection_id = row.try_get("new_connection_id").unwrap_or(-2);
                }
                Err(e) => {
                    eprintln!("Login failed : {}", e);
                    new_connection_id = -3;
                }
            }

            new_connection_id
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            -1
        }};

    Ok(connection_id)
}


/// <- ../login_proc_db_v2/src/main.rs
#[server()]
async fn logout(
    connection_id: i32
) -> Result<String, ServerFnError>
{
    use tokio_postgres::NoTls;

    let res_client = tokio_postgres::connect("host=localhost port=5433 user=alex password=pwd dbname=mydatabase", NoTls).await;

    let reply = match res_client { // Result<String, ServerFnError> to send to the client
        Ok((mut client, connection)) => {
            // Logout call itself

            // Spawn 'connection'.
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error during logout: {}", e);
                }
            });

            let query_logout = format!("call logout(session_id => {});", connection_id);
            let res_logout = client.execute(&query_logout, &[]).await;

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


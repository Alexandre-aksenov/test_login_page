use dioxus::prelude::*;

use sha3_rust::sha3_256; // for hashing the pwd in the DX client
use hex;

// use dioxus_shareables::{shareable, List, ListEntry};
// TOADD: backend, which communicates with PostgreSQL

#[cfg(feature = "server")]
use axum::extract::Query;

/*
/// Copied from the lesson.
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    // let address = dioxus::cli_config::fullstack_address_or_localhost();
    // let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // axum::serve(listener).await.unwrap();
    // -> error[E0061]: this function takes 2 arguments but 1 argument was supplied
    // If this line is commented out: [dev] Application [server] exited gracefully.

    // Fix by RR:
    // axum::serve(listener, axum::Router::new()).await.unwrap();

    dioxus::launch(App);
}
*/

/*
A fn like this is present in the code of the lesson,
but a part of it seems not to be needed for this project.

It contains:
    creation of a table (I did that separately in psql),
    connection to Google (not done for now),
    axum::serve(listener, router).await.unwrap() : this may in fact be necessary.
*/

// const FAVICON: Asset = asset!("/assets/favicon.ico");
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
    rsx! {
        div { "Sign up" } // ADAPT the CLI 'signup'
    }
}

// fn Login() -> Element {
#[component]
fn Login() -> Element {
    let mut signed_in = use_signal(|| false);
    let mut user_login = use_signal(|| String::new());
    let mut cleartext_pwd = use_signal(|| String::new());
    let mut button_text = use_signal(|| String::new());

    // let mut response_msg = use_signal(|| String::new());
    // ->
    let mut response_msg: Signal<String> = use_signal(|| String::from("Please, sign in"));


    if signed_in() == false {
        button_text.set(String::from("Sign in")); // button appears on the 1st load
    } else {
        button_text.set(String::from("Sign out"));
    }

    rsx! {
            div {
                class : "msg",
                "{response_msg}"
            }

        if signed_in() == false {
            div {
                class: "row col2", // these fields can look better after changes in CSS.
                div {
                    input {
                        type : "text",
                        placeholder: "Login",
                        oninput: move |event| {
                            user_login.set(event.value());
                        },
                    }
                }
                div {
                    input {
                        type : "password",
                        placeholder: "Password",
                        oninput: move |event| {
                            cleartext_pwd.set(event.value());
                        },
                    }
                }
            },
        }

        // Button "Sign up"/"Sign in" described by the future var 'button_text'. TOADAPT
        // Call the server fn, TODO
        // -> response_msg
        div {
                button {
                    class : "btn-primary",
                    onclick : move |_| async move {
                        if signed_in() == false {
                            // Sign in user
                            // Calls the server fn 'login'
                            // let response_text = register(first_name(), last_name(), email(), password()).await.unwrap();
                            // ->
                            // hash pwd TOCHECK.  RR's suggestion:
                            // let hashed_pwd = hash_pwd(password());
                            // -> (RR does not provide anything, even type inference, until compilation ?!)
                            let hashed_pwd = hex::encode(sha3_256(cleartext_pwd().as_bytes()));

                            // let response = login(user_login(), hashed_pwd).await.unwrap();
                            // Added a readable reaction in case of failure, such as click without credentials.
                            // ->
                            let res_response = login(user_login(), hashed_pwd).await;
                            /*
                            let response = match res_response {
                                Ok(response) => response,
                                Err(_) => -3,
                            };
                            */
                            let response = res_response.unwrap_or(-4);

                            // response_msg.set(response_text);
                            // ->
                            let response_text = match (response > 0) {
                                true => format!("Login successful, connection id {}", response), // "Login successful".to_string(),
                                false => format!("Login failed, code {}", response), // "Login failed".to_string(),
                            };
                            response_msg.set(response_text);

                        } else {
                            // Sign out user
                            // Will call the server fn 'logout' .
                            // TODO in future (this instruction is here just to avoid compilation errors)
                            let response = login(user_login(), cleartext_pwd()).await.unwrap();

                            response_msg.set(format!("{}", response));
                        }
                    },
                    {button_text()}
                }
            }


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



// Server-side code. TOADAPT from the previous examples.
//
/*
#[server()]
async fn register(
    login: &str,
    hash_pwd: &str
) -> Result<String, ServerFnError>
*/

/// <- ../login_proc_db_v2/src/main.rs
/// new connection_id = -1 should be returned if error on connection to DB.
/// new connection_id = -2 should be returned if the row does not contain "new_connection_id"
///    (which it should according to the signature of the DB procedure).
/// new connection_id = -3 should be returned if login is rejected by DB.
#[server()]
async fn login(
    user_login: String, // &str -> error[E0521]: borrowed data escapes outside of function
    hash_pwd: String
) -> Result<i32, ServerFnError>
{
    // use postgres::{Client, NoTls};
    // ->
    use tokio_postgres::NoTls;

    // let res_client = Client::connect("host=localhost port=5433 user=alex password=pwd dbname=mydatabase", NoTls);
    // ->
    let res_client = tokio_postgres::connect("host=localhost port=5433 user=alex password=pwd dbname=mydatabase", NoTls).await;

    let connection_id = match res_client {
        // Ok(mut client) => {
        Ok((mut client, connection)) => {

            // Suggested by Junie (1st answer of 15-16/8/2026
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });


            let query_login = format!("call sign_in(login => '{}', pwd => '{}');", &user_login, &hash_pwd);
            // let res_login = client.query_one(&query_login, &[]);
            // ->
            let res_login = client.query_one(&query_login, &[]).await;

            let mut new_connection_id: i32 = 0;

            match res_login {
                Ok(row) => {
                    // new_connection_id = row.get("new_connection_id"); // ->
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


/*
// will not be used in the preliminary version v2.0 (?!)
#[server()]
async fn logout(  // <- ../login_proc_db_v2/src/main.rs
    connection_id: i32
) -> Result<String, ServerFnError>

*/

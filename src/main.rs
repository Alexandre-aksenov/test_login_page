use dioxus::prelude::*;

// use dioxus_shareables::{shareable, List, ListEntry};
// TOADD: backend, which communicates with PostgreSQL

#[cfg(feature = "server")]
use axum::extract::Query;

/*
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    
}

A fn like this is present in the code of the lesson,
but it seems not to be needed for this project.

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
        div { "Sign up" } // ADAPT the CLI test
    }
}

// fn Login() -> Element {
#[component]
fn Login() -> Element {
//fn Login(cx: Scope) -> Element {
    rsx! {
        div { "login" } // ADAPT the CLI login

    }
}

#[component]
fn Game() -> Element {
    rsx! {
        div { "logged-in game" }
    }
}

#[component]
pub fn Home() -> Element {
    crate::Hero()
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" } // Dioxus: interfaces that run anywhere
            div { id: "links",
                a { href: "/trial", "📚 Trial without account" }
                a { href: "/login", "🚀 Login" }
                a { href: "/signup", "📡 Sign up" }
                // a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                // a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                // a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
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

/*
#[server()]
async fn login(  // <- ../login_proc_db_v2/src/main.rs
    login: &str,
    hash_pwd: &str
) -> Result<i32, ServerFnError>  // new connection_id

*/

/*
// will not be used in the preliminary version v2.0 .
#[server()]
async fn logout(  // <- ../login_proc_db_v2/src/main.rs
    connection_id: i32
) -> Result<String, ServerFnError>

*/
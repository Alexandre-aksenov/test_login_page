use dioxus::prelude::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// https://dioxuslabs.com/learn/0.7/essentials/router/#creating-a-routable-enum
#[derive(Routable, Clone, PartialEq)] // manual suggests to add Debug
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

#[component]
fn Login() -> Element {
    rsx! {
        div { "login" } // ADAPT the CLI test
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

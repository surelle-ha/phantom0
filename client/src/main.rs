use dioxus::prelude::*;

const STYLES: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLES }
        main { class: "shell",
            section { class: "hero",
                p { class: "eyebrow", "PHANTOM0 / SIEM PLATFORM" }
                h1 { "Security visibility, ready when you are." }
                p { class: "lede",
                    "The Dioxus dashboard is connected to the Phantom0 platform and ready for its first features."
                }
                div { class: "actions",
                    a { class: "primary", href: "/api/users/", "Check user service" }
                    a { class: "secondary", href: "/api/identity/health", "Platform health" }
                }
            }
            section { class: "services", "aria-label": "Platform services",
                StatusCard { name: "Identity", path: "/api/identity/health", description: "Authentication and access control" }
                StatusCard { name: "Users", path: "/api/users/health", description: "User profiles and preferences" }
                StatusCard { name: "Groups", path: "/api/groups/health", description: "Memberships and group access" }
                StatusCard { name: "Notifications", path: "/api/notifications/health", description: "Security alerts and delivery" }
            }
        }
    }
}

#[component]
fn StatusCard(name: &'static str, path: &'static str, description: &'static str) -> Element {
    rsx! {
        a { class: "service-card", href: path,
            span { class: "status-dot" }
            div {
                h2 { "{name}" }
                p { "{description}" }
            }
            span { class: "arrow", "→" }
        }
    }
}

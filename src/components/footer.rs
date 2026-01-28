use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "w-full border-t border-border mt-12",
            div { class: "max-w-6xl mx-auto px-6 py-6 flex flex-col sm:flex-row items-center justify-between gap-3 text-sm text-muted",
                p { "© Creamy Cream" }
                p { "We keep cookies in the ice cream — not in your browser." }
                p {
                    a {
                        class: "link mr-2",
                        rel: "noopener noreferrer",
                        href: "/ingredients",
                        "· Existing Ingredients"
                    }
                    a {
                        class: "link mr-2",
                        rel: "noopener noreferrer",
                        target: "_blank",
                        href: "https://kg-m3.com",
                        "· Density Database"
                    }
                    a {
                        class: "link mr-2",
                        rel: "noopener noreferrer",
                        target: "_blank",
                        href: "https://github.com/omgMath/creamy-cream",
                        "· GitHub"
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "w-full bg-background/80 backdrop-blur border-b border-border z-40",
            nav { class: "max-w-6xl mx-auto px-6 py-4 flex items-center justify-between",
                a { href: "/", class: "flex items-center gap-3 no-underline",
                    img {
                        class: "size-12",
                        src: asset!("/assets/favicon-152x152.png"),
                        alt: "Creamy Cream logo",
                    }
                    div {
                        h1 { class: "text-3xl font-semibold tracking-tight", "Creamy Cream" }
                        p { class: "text-sm text-secondary -mt-1",
                            "Optimize your ice cream creaminess with science!"
                        }
                    }
                }
                nav { class: "flex items-center gap-4 text-sm",
                    a { class: "nav-link", href: "/ingredients/create", "New ingredient" }
                }
            }
        }
    }
}

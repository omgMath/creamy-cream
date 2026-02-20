use dioxus::prelude::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BarData {
    pub label: String,
    pub value: f64,
}

const COLORS: [&str; 5] = ["blue", "cyan", "success", "warning", "error"];

#[component]
pub fn BarChart(
    data: Vec<BarData>,
    label: String,
    background_data: Vec<BarData>,
    background_label: String,
) -> Element {
    rsx! {
        div { class: "mt-2 flex space-x-4",
            {legend(label.into(), &data, "".into())}
            {legend(background_label.into(), &background_data, "/50".into())}
        }
        div { class: "relative h-8 w-full",
            {bars(&background_data, "h-8 z-10 top-0".into(), "/50".into())}
            {bars(&data, "h-4 z-20 top-2".into(), "".into())}
        }
    }
}

fn bars(data: &[BarData], custom_class: String, color_shade: String) -> Element {
    rsx! {
        div { class: "{custom_class} w-full absolute left-0 flex",
            for (i , item) in data.iter().enumerate() {
                {
                    let class = format!("h-full bg-{}{}", COLORS[i], color_shade);
                    rsx! {
                        div { class, style: format!("width: {}%", item.value) }
                    }
                }
            }
        }
    }
}

fn legend(label: String, data: &[BarData], color_shade: String) -> Element {
    rsx! {
        div { class: "text-xs",
            span {
                b { "{label}" }
            }
            for (i , item) in data.iter().enumerate() {
                div { class: "flex items-center",
                    span { class: "inline-block bg-{COLORS[i]}{color_shade} size-2 rounded-full ml-2 mr-1" }
                    span { "{item.label}: {item.value:.2}%" }
                }
            }
        }
    }
}

use crate::api::ingredient::create_ingredient;
use crate::api::models::IngredientForm;
use dioxus::prelude::*;

#[component]
pub fn IngredientCreate() -> Element {
    let mut name = use_signal(String::new);
    let mut fat = use_signal(|| 0.0);
    let mut sugar = use_signal(|| 0.0);
    let mut water = use_signal(|| 0.0);
    let mut density = use_signal(|| 0.0);
    let mut feedback = use_signal(String::new);
    let mut reset = move || {
        name.set(String::new());
        fat.set(0.0);
        sugar.set(0.0);
        water.set(0.0);
        density.set(0.0);
    };
    let onsubmit = move |evt: FormEvent| async move {
        evt.prevent_default();
        let values = IngredientForm {
            name: name().clone(),
            fat: fat(),
            sugar: sugar(),
            water: water(),
            density: density(),
        };
        match create_ingredient(values).await {
            Ok(_) => {
                let created_name = name().clone();
                reset();
                feedback.set(format!(
                    "🎉 Ingredient '{}' created successfully!",
                    created_name
                ));
            }
            Err(e) => {
                feedback.set(format!("❌ Error creating ingredient: {}", e));
            }
        }
    };

    rsx! {
        div { class: "mx-auto max-w-2xl px-6 py-8 bg-surface-elevated rounded-lg",
            if feedback().len() > 0 {
                p { class: "border rounded border-success px-2 py-1 mb-4 text-sm",
                    "{feedback}"
                }
            }
            h1 { class: "text-3xl font-semibold tracking-tight mb-4", "Add new ingredient" }
            form { class: "flex flex-col gap-4", onsubmit,
                // Name
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm text-text-secondary", r#for: "name", "Name" }
                    input {
                        class: "bg-surface border border-border rounded px-3 py-2 text-sm text-text-primary placeholder-text-muted focus:ring-2 focus:ring-accent-secondary focus:outline-none transition duration-fast",
                        r#type: "text",
                        id: "name",
                        name: "Name",
                        required: true,
                        value: name,
                        oninput: move |e| name.set(e.parsed().expect("Parsing failed")),
                    }
                }
                // Fat / Sugar / Water (percent inputs)
                for (label , mut refe) in [(&"Fat", fat), (&"Sugar", sugar), (&"Water", water)].iter().cloned() {
                    div { class: "flex flex-col gap-1",
                        label {
                            class: "text-sm text-text-secondary",
                            r#for: label.to_lowercase(),
                            {label.to_string() + " (in %)"}
                        }
                        input {
                            class: "bg-surface border border-border rounded px-3 py-2 text-sm text-text-primary font-mono placeholder-text-muted focus:ring-2 focus:ring-accent-secondary focus:outline-none transition duration-fast",
                            r#type: "number",
                            id: label.to_lowercase(),
                            name: label.to_string() + " (in %)",
                            required: true,
                            min: "0",
                            max: "100",
                            step: "0.01",
                            value: refe(),
                            oninput: move |e| refe.set(e.parsed().expect("Parsing failed")),
                        }
                    }
                }
                // Density
                div { class: "flex flex-col gap-1",
                    label {
                        class: "text-sm text-text-secondary",
                        r#for: "density",
                        "Density (in kg/m³)"
                    }
                    input {
                        class: "bg-surface border border-border rounded px-3 py-2 text-sm text-text-primary font-mono placeholder-text-muted focus:ring-2 focus:ring-accent-secondary focus:outline-none transition duration-fast",
                        r#type: "number",
                        id: "density",
                        name: "Density (in kg/m³)",
                        required: true,
                        min: "0",
                        step: "0.01",
                        value: density(),
                        oninput: move |e| density.set(e.parsed().expect("Parsing failed")),
                    }
                }
                div { class: "flex gap-3 mt-4",
                    button { class: "button-primary", r#type: "submit", "Create Ingredient" }
                    button {
                        class: "button-secondary",
                        r#type: "button",
                        onclick: move |_| reset(),
                        "Reset"
                    }
                }
            }

        }
    }
}

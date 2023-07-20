use leptos::*;

// TODO: see if you can validate Children, being a String or Input.
// Maybe use: https://docs.rs/leptos/latest/leptos/struct.NodeRef.html
#[component]
fn Label(
    cx: Scope,
    #[prop(into)] 
    html_for: String,
    children: Children
) -> impl IntoView {
    view! { cx,
        <label
            for=html_for
            on:mousedown=move |ev| {
                // prevent text selection when double clicking label
                if ev.default_prevented() && ev.detail() > 1 {
                    ev.prevent_default();
                }
            }
        >
            {children(cx)}
        </label>
    }
}

#[component]
pub fn LabelPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Label"</h1>
        <h2>"Example"</h2>
        <Label html_for="first-name">"First name"</Label>
        <input
            id="first-name"
            class=("p-1 border border-slate-500")
        />

    }
}


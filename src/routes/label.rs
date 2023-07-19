use leptos::*;

// TODO: see if you can validate Children, being a String or Input.
// Maybe use: https://docs.rs/leptos/latest/leptos/struct.NodeRef.html
#[component]
fn Label(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <label>
            {children(cx)}
        </label>
    }
}

#[component]
pub fn LabelPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Label"</h1>
        <h2>"Example"</h2>
        <Label>"First name"</Label>
        <input
            id="first-name"
            class=("p-1 border border-slate-500")
        />

    }
}


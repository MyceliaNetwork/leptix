use leptos::*;

#[component]
fn Label(
    cx: Scope,
    
    #[prop(default = None, optional, into)]
    html_for: Option<String>,
    
    #[prop(default = None, optional, into)]
    class: Option<AttributeValue>,    

    children: Children // TODO: specify what types of Children are allowed
) -> impl IntoView {
    view! { cx,
        <label
            class=class
            for=html_for
            on:mousedown=move |ev| {
                // prevent text selection when double clicking label
                if !ev.default_prevented() && ev.detail() > 1 {
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

        <div>
            <Label html_for="first-name">"For attribute"</Label>
            <input
                id="first-name"
                class=("p-1 border border-slate-500")
            />
        </div>

        <Label>
            "Nested Label"
            <input type="text" class=("p-1 border border-slate-500") />
        </Label>

        <h2>"Features"</h2>
        <ul>
            <li>"Text selection is prevented when double clicking label."</li>
            <li>"Supports nested controls."</li>
        </ul>
    }
}
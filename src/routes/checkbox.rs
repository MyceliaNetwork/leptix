use leptos::*;

#[component]
fn Checkbox (
    cx: Scope,

    #[prop(default = None, optional, into)]
    class: Option<AttributeValue>,

    #[prop(into)]
    checked: Signal<bool>
    
) -> impl IntoView {
    view! { cx,
        <input
            type="checkbox"
            class=class
            prop:value=checked
        />
    }
}

#[component]
pub fn CheckboxPage(cx: Scope) -> impl IntoView {
    let (checked, set_checked) = create_signal(cx, false);

    view! { cx,
        <h1>"Checkbox"</h1>
        <h2>"Example"</h2>
        <Checkbox
            checked={checked}
            on:change=move |ev| {
                set_checked(event_target_checked(&ev));
            }
        />
    }
}
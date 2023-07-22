use leptos::*;

#[component]
fn Checkbox (
    cx: Scope,

    #[prop(default = None, optional, into)]
    class: Option<AttributeValue>,

    // #[prop(into)]
    // disabled: ReadSignal<bool>,
    #[prop(optional, into)]
    disabled: Option<AttributeValue>,

    #[prop(into)]
    checked: ReadSignal<bool>,
    
) -> impl IntoView {
    let mut disabled = disabled;
    if let None = disabled {
        disabled = Some(Box::new(Attribute::Bool(false)))
    }

    let disabled = disabled.unwrap();
    let disabled = disabled.into_attribute_boxed(cx);
    view! { cx,
        <input
            type="checkbox"
            class=class
            disabled=disabled
            prop:value=checked
        />
    }
}

#[component]
pub fn CheckboxPage(cx: Scope) -> impl IntoView {
    let (checked, set_checked) = create_signal(cx, true);
    let (disabled, set_disabled) = create_signal(cx, false);

    view! { cx,
        <h1>"Checkbox"</h1>
        <h2>"Example"</h2>
        <button
            type="button"
            on:click=move |_| {
                set_disabled(!disabled.get())
            }
        >
            "Toggle disabled"
        </button>

        <Checkbox
            checked=checked
            disabled=disabled
            on:change=move |ev| {
                set_checked(event_target_checked(&ev));
            }
        />
    }
}
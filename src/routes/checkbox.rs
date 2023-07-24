use leptos::*;

#[component]
fn BubbleCheckbox (
    cx: Scope,

    #[prop(default = None, optional, into)]
    class: Option<AttributeValue>,

    #[prop(optional, into)]
    disabled: Option<AttributeValue>,

    #[prop(optional, into)]
    required: Option<AttributeValue>,

    #[prop(into)]
    checked: ReadSignal<bool>,
    
) -> impl IntoView {
    // disabled
    let mut disabled = disabled;
    if let None = disabled {
        disabled = Some(Box::new(Attribute::Bool(false)))
    }
    let disabled = disabled.unwrap();
    let disabled = disabled.into_attribute_boxed(cx);

    // required
    let mut required = required;
    if let None = required {
        required = Some(Box::new(Attribute::Bool(false)))
    }
    let required = required.unwrap();
    let required = required.into_attribute_boxed(cx);

    let value = if checked.get() { "on" } else { "off" };
 
    view! { cx,
        <button
            type="button"
            role="checkbox"
            aria-required=required.clone()
            // TODO: add indeterminate state
            aria-checked=""
            data-disabled=disabled.clone()
        />
        <input
            type="checkbox"
            aria-hidden
            class=class
            disabled=disabled
            required=required
            prop:value=checked
            tab-index="-1"
            value=value
            // TODO: uncomment when we have an Indicator component
            // style="position: 'absolute'; pointer_events: 'none'; opacity: 0; margin: 0;"
        />
    }
}

#[component]
pub fn CheckboxPage(cx: Scope) -> impl IntoView {
    let (checked, set_checked) = create_signal(cx, true);
    let (disabled, set_disabled) = create_signal(cx, false);
    let (required, set_required) = create_signal(cx, false);

    view! { cx,
        <h1>"Checkbox"</h1>
        <h2>"Example"</h2>
        
        <div>
            <button
                type="button"
                on:click=move |_| {
                    set_disabled(!disabled.get())
                }
            >
                "Toggle disabled"
            </button>
        </div>

        <div>
            <button
                type="button"
                on:click=move |_| {
                    set_required(!required.get())
                }
            >
                "Toggle required"
            </button>
        </div>

        <BubbleCheckbox
            checked=checked
            disabled=disabled
            required=required
            on:change=move |ev| {
                set_checked(event_target_checked(&ev));
            }
        />
    }
}
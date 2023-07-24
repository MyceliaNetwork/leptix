use leptos::*;
// use crate::routes::checkbox::leptos_dom::console_log;

// const INDETERMINATE: &'static str = "indeterminate";

struct CheckedBool {
    True: Bool,
    False: Bool
}

struct Indeterminate<'a> {
    value: &'a str
}

enum CheckedValue<'a> {
    Bool(CheckedBool),
    Indeterminate(Indeterminate<'a>),
}

fn test<'a>(v: CheckedValue<'a>) {
    match v {
        CheckedValue::Bool( CheckedBool { width: w, height: h }) =>
            println!("CheckedValue::Bool:{}, h:{}", w, h),
        CheckedValue::Indeterminate( Indeterminate { value: u }) =>
            println!("Indeterminate :{}", u),
    }
}



#[component]
fn BubbleCheckbox (
    cx: Scope,

    #[prop(optional, into)]
    class: Option<AttributeValue>,

    #[prop(optional, into)]
    disabled: Option<AttributeValue>,

    #[prop(optional, into)]
    required: Option<AttributeValue>,

    #[prop()]
    checked: Signal<CheckedState>,
    
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

    // FIXME: still renders to "true" in the DOM. Most likely a Leptos issue
    // let value = if checked.get().clone() { "on" } else { "off" };
    // console_log(value);
 
    view! { cx,
        <button
            type="button"
            role="checkbox"
            aria-required=required.clone()
            // TODO: add indeterminate state
            aria-checked=checked.as_dom_value()
            data-disabled=disabled.clone()
            // TODO: add composeEventHandlers
            on:keydown=move |ev| {
                // According to WAI ARIA, Checkboxes don't activate on enter keypress
                if ev.key() == "Enter" {
                    ev.prevent_default();
                }
            }
        />
        <input
            type="checkbox"
            aria-hidden
            class=class
            disabled=disabled
            required=required
            tab-index="-1"
            // TODO: uncomment when we have an Indicator component
            // style="position: 'absolute'; pointer_events: 'none'; opacity: 0; margin: 0;"
        />
    }
}

#[component]
pub fn CheckboxPage(cx: Scope) -> impl IntoView {
    let indeterminate: &'static CheckedValue = &CheckedValue::Indeterminate; // "Indeterminate"
    // let c: Boolean = CheckedValue::Boolean.to_boolean(); // 
    let (checked, set_checked) = create_signal(cx, indeterminate);
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
                event_target_checked(&ev);
                let value = if &ev { CheckedValue::True } else { CheckedValue::False };
                set_checked(&value);
            }
        />
    }
}
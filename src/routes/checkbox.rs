use leptos::*;
// use crate::routes::checkbox::leptos_dom::console_log;

const INDETERMINATE: &str = "indeterminate";

pub enum BoolOrString {
    Bool(bool),
    Str(String),
}

// Implement functions for the enum
impl BoolOrString {
    pub fn get_value(&self) -> &str {
        match self {
            BoolOrString::Bool(b) => {
                if *b {
                    "true"
                } else {
                    "false"
                }
            },
            BoolOrString::Str(s) => INDETERMINATE,
        }
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

    // #[prop(into)]
    // checked: ReadSignal<bool>,
    
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

    // let true_val = BoolOrString::Bool(true);
    // let false_val = BoolOrString::Bool(false);
    let indeterminate_val = BoolOrString::Str(INDETERMINATE.to_string());
 
    view! { cx,
        <button
            type="button"
            role="checkbox"
            aria_required=required.clone()
            // TODO: add indeterminate state
            aria_checked="indeterminate"
            data_disabled=disabled.clone()
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
            checked="indeterminate"
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
    // let (checked, set_checked) = create_signal(cx, false);
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
            // checked=checked
            disabled=disabled
            required=required
            // on:change=move |ev| {
            //     // let value = if &ev { CheckedValue::True } else { CheckedValue::False };
            //     set_checked(event_target_checked(&ev));
            // }
        />
    }
}
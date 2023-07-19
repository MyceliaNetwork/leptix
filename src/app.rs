use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::routes::home::HomePage;
use crate::routes::button::ButtonPage;
use crate::routes::checkbox::CheckboxPage;
use crate::routes::label::LabelPage;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptix.css"/>

        // sets the document title
        <Title text="Leptix - Leptos Radix UI port"/>

        // content for this welcome page
        <Router>
            <Nav />
            <main class=("prose p-4")>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/label" view=LabelPage/>
                    <Route path="/button" view=ButtonPage/>
                    <Route path="/checkbox" view=CheckboxPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    view! { cx,
        <header>
            <nav class=("p-4 flex gap-8")>
                <A href="/">
                    "Home"
                </A>
                <A href="/button">
                    "Button"
                </A>
                <A href="/checkbox">
                    "Checkbox"
                </A>
                <A href="/label">
                    "Label"
                </A>
                <a class="github" href="github.com/departurelabsic/" target="_blank" rel="noreferrer">
                    "Built by Mycelia"
                </a>
            </nav>
        </header>
    }
}


/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}

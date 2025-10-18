use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-actix-diesel-postgres-tailwind.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
            // Hero Section
            <nav class="container mx-auto px-6 py-4">
                <div class="flex justify-between items-center">
                    <div class="text-2xl font-bold text-indigo-600">"MyApp"</div>
                    <div class="space-x-4">
                        <a href="#features" class="text-gray-700 hover:text-indigo-600 transition">
                            "Features"
                        </a>
                        <a href="#about" class="text-gray-700 hover:text-indigo-600 transition">
                            "About"
                        </a>
                        <button class="bg-indigo-600 text-white px-6 py-2 rounded-lg hover:bg-indigo-700 transition">
                            "Get Started"
                        </button>
                    </div>
                </div>
            </nav>

            // Hero Content
            <div class="container mx-auto px-6 py-20">
                <div class="text-center max-w-3xl mx-auto">
                    <h1 class="text-5xl md:text-6xl font-bold text-gray-900 mb-6">
                        "Build Fast, Modern Web Apps"
                    </h1>
                    <p class="text-xl text-gray-600 mb-8">
                        "Powered by Leptos, Actix, Diesel, and PostgreSQL. The perfect stack for building high-performance full-stack applications."
                    </p>
                    <div class="flex justify-center space-x-4">
                        <button class="bg-indigo-600 text-white px-8 py-3 rounded-lg text-lg font-semibold hover:bg-indigo-700 transition shadow-lg">
                            "Start Building"
                        </button>
                        <button class="bg-white text-indigo-600 px-8 py-3 rounded-lg text-lg font-semibold hover:bg-gray-50 transition shadow-lg border border-indigo-600">
                            "Learn More"
                        </button>
                    </div>
                </div>
            </div>

            // Features Section
            <div id="features" class="container mx-auto px-6 py-20">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">"Features"</h2>
                <div class="grid md:grid-cols-3 gap-8">
                    <div class="bg-white p-8 rounded-xl shadow-lg hover:shadow-xl transition">
                        <div class="text-indigo-600 text-4xl mb-4">"⚡"</div>
                        <h3 class="text-2xl font-bold text-gray-900 mb-3">"Lightning Fast"</h3>
                        <p class="text-gray-600">
                            "Built with Rust for maximum performance and reliability. Zero-cost abstractions and compile-time guarantees."
                        </p>
                    </div>
                    <div class="bg-white p-8 rounded-xl shadow-lg hover:shadow-xl transition">
                        <div class="text-indigo-600 text-4xl mb-4">"🔒"</div>
                        <h3 class="text-2xl font-bold text-gray-900 mb-3">"Type Safe"</h3>
                        <p class="text-gray-600">
                            "End-to-end type safety from database to UI. Catch errors at compile time, not runtime."
                        </p>
                    </div>
                    <div class="bg-white p-8 rounded-xl shadow-lg hover:shadow-xl transition">
                        <div class="text-indigo-600 text-4xl mb-4">"🚀"</div>
                        <h3 class="text-2xl font-bold text-gray-900 mb-3">"Full Stack"</h3>
                        <p class="text-gray-600">
                            "Complete solution with server-side rendering, database ORM, and reactive UI in one coherent stack."
                        </p>
                    </div>
                </div>
            </div>

            // Footer
            <footer class="bg-gray-900 text-white py-12">
                <div class="container mx-auto px-6 text-center">
                    <p class="text-gray-400">"Built with Leptos + Actix + Diesel + PostgreSQL"</p>
                    <p class="text-gray-500 mt-2">"© 2025 All rights reserved"</p>
                </div>
            </footer>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
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
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}

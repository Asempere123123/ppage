use yew::prelude::*;
use yew_router::prelude::*;
use yew_bootstrap::util::*;

mod pages;
mod page_structure;

use page_structure::PageStructure;
use pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <PageStructure><Home /></PageStructure> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
    <>
        {include_cdn()}
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        {include_cdn_js()}
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
mod components;
mod pages;
mod router;

use router::AppRouter;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <AppRouter />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}

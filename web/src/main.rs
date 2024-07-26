mod components;
mod contexts;
mod pages;
mod router;

use router::AppRouter;
use yew::prelude::*;

use crate::contexts::history::HistoryProvider;

#[function_component(App)]
fn app() -> Html {
    html! {
        <HistoryProvider>
            <AppRouter />
        </HistoryProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}

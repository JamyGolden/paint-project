use crate::components::header::Header;
use yew::prelude::*;

pub fn search_results() -> Html {
    html! {<>
        <Header />
        <h1>{ "Search results" }</h1>
    </>}
}

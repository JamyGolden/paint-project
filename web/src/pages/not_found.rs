use crate::components::header::Header;
use yew::prelude::*;

pub fn not_found() -> Html {
    html! {<>
        <Header />
        <h1>{ "404 not found" }</h1>
    </>}
}

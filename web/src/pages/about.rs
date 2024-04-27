use crate::components::header::Header;
use yew::prelude::*;

pub fn about() -> Html {
    html! {<>
        <Header />
        <h1>{ "About us" }</h1>
    </>}
}

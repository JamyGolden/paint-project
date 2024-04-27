use crate::components::header::Header;
use yew::prelude::*;

pub fn home() -> Html {
    html! {<>
        <Header />
        <h1>{ "Home" }</h1>
    </>}
}

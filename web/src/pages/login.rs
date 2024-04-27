use crate::components::header::Header;
use yew::prelude::*;

pub fn login() -> Html {
    html! {<>
        <Header />
        <h1>{ "Login" }</h1>
    </>}
}

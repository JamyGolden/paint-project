use crate::components::header::Header;
use yew::prelude::*;

pub fn signup() -> Html {
    html! {<>
        <Header />
        <h1>{ "Sign up" }</h1>
    </>}
}

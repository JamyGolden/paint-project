use crate::components::header::Header;
use yew::prelude::*;

pub fn terms() -> Html {
    html! {<>
        <Header />
        <h1>{ "Terms & Conditions" }</h1>
    </>}
}

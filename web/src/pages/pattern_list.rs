use crate::components::header::Header;
use yew::prelude::*;

pub fn pattern_list() -> Html {
    html! {<>
        <Header />
        <h1>{ "Pattern List" }</h1>
    </>}
}

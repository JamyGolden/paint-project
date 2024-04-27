use crate::components::header::Header;
use yew::prelude::*;

pub fn user_dashboard() -> Html {
    html! {<>
        <Header />
        <h1>{ "Dashboard" }</h1>
        <p>{ "Can contain pinned list, following list, collections, etc" }</p>
    </>}
}

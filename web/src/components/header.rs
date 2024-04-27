use crate::components::nav::Nav;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="Header">
            <Nav />
        </div>
    }
}

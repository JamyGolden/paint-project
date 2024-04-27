use crate::components::header::Header;
use yew::prelude::*;

pub fn privacy() -> Html {
    html! {<>
        <Header />
        <h1>{ "Privacy" }</h1>
    </>}
}

use crate::components::header::Header;
use yew::prelude::*;

pub fn contact() -> Html {
    html! {<>
        <Header />
        <h1>{ "Contact" }</h1>

        <iframe height="800" title="Contact Form" allowtransparency="true" frameborder="0" scrolling="no" style="width:100%;border:none" sandbox="allow-popups-to-escape-sandbox allow-top-navigation allow-scripts allow-popups allow-forms allow-same-origin" src="https://jamygolden.wufoo.com/embed/q1kkuj030p1cxcr/">
            <a href="https://jamygolden.wufoo.com/forms/q1kkuj030p1cxcr/">{ "Fill out my Wufoo form!" }</a>
        </iframe>
    </>}
}

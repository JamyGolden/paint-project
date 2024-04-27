use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {<>
        <ul class="Nav">
            <li class="Nav-item"><a class="Nav-link" href="/">{ "Home" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/about">{ "About" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/contact">{ "Contact" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/pattern-detail">{ "Pattern Detail" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/pattern-list">{ "Pattern List" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/privacy">{ "Privacy" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/search-results">{ "Search Results" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/terms">{ "Terms" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/dashboard">{ "Dashboard" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/pattern-editor">{ "Pattern Editor" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/login">{ "Login" }</a></li>
            <li class="Nav-item"><a class="Nav-link" href="/signup">{ "Signup" }</a></li>
        </ul>
    </>}
}

use crate::pages;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/login")]
    Login,
    #[at("/pattern-detail/:id")]
    PatternDetail { id: String },
    #[at("/pattern-list")]
    PatternList,
    #[at("/privacy")]
    Privacy,
    #[at("/search-results")]
    SearchResults,
    #[at("/signup")]
    Signup,
    #[at("/terms")]
    Terms,
    #[at("/dashboard")]
    UserDashboard,
    #[at("/pattern-editor")]
    UserPatternEditor,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::About => pages::about::about(),
        Route::Contact => pages::contact::contact(),
        Route::Home => pages::home::home(),
        Route::Login => pages::login::login(),
        Route::PatternDetail { id } => pages::pattern_detail::pattern_detail(id),
        Route::PatternList => pages::pattern_list::pattern_list(),
        Route::Privacy => pages::privacy::privacy(),
        Route::SearchResults => pages::search_results::search_results(),
        Route::Signup => pages::signup::signup(),
        Route::Terms => pages::terms::terms(),
        Route::UserDashboard => pages::user_dashboard::user_dashboard(),
        Route::UserPatternEditor => pages::user_pattern_editor::user_pattern_editor(),
        _ => pages::not_found::not_found(), // This catches all unmatched routes
    }
}

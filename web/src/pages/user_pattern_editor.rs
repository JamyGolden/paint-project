pub mod components;
pub mod containers;
pub mod reducer;

use yew::prelude::*;

use crate::components::header::Header;

use containers::editor::PatternEditor;

#[derive(Clone, Debug, PartialEq)]
pub enum PatternEditorFeature {
    Brush,
    Fill,
    Pointer,
}

pub fn user_pattern_editor() -> Html {
    html! {<>
        <Header />
        <h1>{ "Pattern editor" }</h1>
        <PatternEditor />
    </>}
}

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::pattern::Rgb;

use super::super::PatternEditorFeature;

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub on_color_change: Callback<String>,
    pub on_feature_change: Callback<PatternEditorFeature>,
    pub on_undo: Callback<MouseEvent>,
    pub default_color: Rgb,
}
#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let ToolbarProps {
        on_color_change,
        on_feature_change,
        on_undo,
        default_color,
    } = props;
    let color_handle = use_state(|| default_color.as_hex());
    let color = (*color_handle).clone();
    let on_color_change = on_color_change.clone();

    let handle_color_change = {
        let color_handle = color_handle.clone();
        let on_color_change = on_color_change.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                let hex_value = input.value();
                color_handle.set(hex_value.clone());
                on_color_change.emit(hex_value);
            }
        })
    };

    html! {<>
            <div>
                <label>
                    { "thread color" }
                    <input type="color" name="brush_color" value={color} onchange={handle_color_change} />
                </label>
                <div>
                    <button onclick={create_handle_feature_change(PatternEditorFeature::Brush, on_feature_change.clone())}>{ "Brush" }</button>
                </div>
                <div>
                    <button onclick={create_handle_feature_change(PatternEditorFeature::Fill, on_feature_change.clone())}>{ "Fill" }</button>
                </div>
                <div>
                    <button onclick={create_handle_feature_change(PatternEditorFeature::Pointer, on_feature_change.clone())}>{ "Pointer" }</button>
                </div>
                <div>
                    <button onclick={on_undo}>{ "Undo" }</button>
                </div>
            </div>
    </>}
}
fn create_handle_feature_change(
    feature: PatternEditorFeature,
    callback: Callback<PatternEditorFeature>,
) -> Callback<MouseEvent> {
    Callback::from(move |_: MouseEvent| {
        callback.emit(feature.clone());
    })
}

use gloo::{events::EventListener, utils::window};
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props<T: PartialEq> {
    pub render: Box<T>,
    pub children: Html,
    pub style: Option<String>,
    pub width: usize,
    pub height: usize,
    #[prop_or_default]
    pub onmousedown: Callback<MouseEvent>,
    #[prop_or_default]
    pub onmousemove: Callback<MouseEvent>,
    #[prop_or_default]
    pub node_ref: NodeRef,
}

/// A Canvas component is encapsulated.
///
/// # Parameters and types
/// ```ignore
/// <Canvas<...1, ...2>
///    style="
///        ...3
///    "
///    render={Box::new(...4)}
/// />
/// ```
/// **...1:** The canvas context u need.
///
/// **...2:** struct you impl`yew_canvas::WithRender`.
///
/// **...3:** Just use style, canvas can suit automaticly.
///
/// **...4:** U have to wrap ur `yew_canvas::WithRender` struct in `Box<>`.
///
/// # Example
///
/// ```ignore
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html!(
///         <Canvas<CanvasRenderingContext2d, Render>
///             //Just use style, canvas can suit automaticly.
///             style="
///                 width: 100%;
///                 height: 100%;
///             "
///             render={Box::new(Render())}
///         />
///             {"The browser is not supported."}
///         </Canvas<CanvasRenderingContext2d, Render>>
///     )
/// }
/// ```
#[function_component(Canvas)]
pub fn canvas<CanvasContext, T>(props: &Props<T>) -> Html
where
    T: PartialEq + WithRender + Clone + 'static,
    CanvasContext: JsCast,
{
    let node_ref = props.node_ref.clone();
    let is_first_render = use_state(|| true);
    let style = props.style.clone().unwrap_or_default();
    let display_size = use_state(|| (props.width, props.height));
    let size_listen_enent_state = use_state(|| EventListener::new(&window(), "resize", |_| ()));

    {
        let node_ref = node_ref.clone();
        let display_size = display_size.clone();
        let render = props.render.clone();

        use_effect(move || {
            if let Some(canvas) = node_ref.cast::<HtmlCanvasElement>() {
                if *is_first_render {
                    is_first_render.set(false);
                    let canvas = canvas.clone();

                    display_size.set((
                        canvas.client_width() as usize,
                        canvas.client_height() as usize,
                    ));

                    size_listen_enent_state.set(EventListener::new(
                        &window(),
                        "resize",
                        move |_| {
                            display_size.set((
                                canvas.client_width() as usize,
                                canvas.client_height() as usize,
                            ));
                        },
                    ));
                }

                render.render(&canvas);
            }

            || ()
        });
    }

    let children = props.children.clone();

    html! {
    <canvas
        style={style}
        width={display_size.clone().deref().0.to_string()}
        height={display_size.deref().1.to_string()}
        ref={node_ref}
        onmousedown={props.onmousedown.clone()}
        onmousemove={props.onmousemove.clone()}
    >
        {children.clone()}
    </ canvas>
    }
}

/// Implement this trait for rendering.
///
/// use `&self` to pass data.
///
/// # example
/// ```ignore
/// #[derive(Clone, PartialEq)]
///struct Render();
///
///impl WithRender for Render {
///    fn render(self, canvas: &HtmlCanvasElement) {
///    // CanvasRenderingContext2d can be
///    // any kind of canvas context.
///    // Make sure that, it's the same
///    // context as Canvas component.
///        let interface: CanvasRenderingContext2d = canvas
///            .get_context("2d")
///            .unwrap()
///            .unwrap()
///            .dyn_into()
///            .unwrap();
///    ...
/// ```
pub trait WithRender: Clone + PartialEq {
    fn render(self, canvas: &HtmlCanvasElement);
}

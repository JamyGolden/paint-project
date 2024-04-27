use implicit_clone::sync::IArray;
use log::error;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::pattern::{GridCell, Rgb, Thread, ThreadType};

fn create_on_change_event(
    thread_type: ThreadType,
    thread_cell: (GridCell, IArray<Thread>),
    callback: &Callback<(GridCell, IArray<Thread>)>,
) -> Callback<Event> {
    let callback = callback.clone();
    let (GridCell(col_index, row_index), threads) = thread_cell;

    Callback::from(move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>();

        if let Some(input) = input {
            let value = input.value();
            let color = Rgb::from_hex(value.clone());

            match color {
                Ok(color) => {
                    if !value.is_empty() {
                        let thread_type_exists =
                            threads.iter().any(|t| t.thread_type == thread_type);

                        if thread_type_exists {
                            let threads: Vec<Thread> = threads
                                .iter()
                                .map(|t| {
                                    if t.thread_type == thread_type {
                                        Thread::new(color, thread_type)
                                    } else {
                                        t
                                    }
                                })
                                .collect();

                            let updated_thread_cell: (GridCell, IArray<Thread>) =
                                (GridCell(col_index, row_index), IArray::from(threads));
                            callback.emit(updated_thread_cell);
                        } else {
                            let mut threads: Vec<Thread> = vec![Thread::new(color, thread_type)];
                            threads.extend(threads.to_vec());

                            let updated_thread_cell: (GridCell, IArray<Thread>) =
                                (GridCell(col_index, row_index), IArray::from(threads));
                            callback.emit(updated_thread_cell);
                        }
                    }
                }
                Err(err) => error!("{}", err),
            }
        }
    })
}

fn find_thread_type_hex_value(
    thread_type: ThreadType,
    threads: &[Thread],
    default_color: Rgb,
) -> String {
    threads
        .iter()
        .find_map(|thread| {
            if thread.thread_type == thread_type {
                Some(thread.color.as_hex())
            } else {
                None
            }
        })
        .unwrap_or(default_color.as_hex())
}

#[derive(Properties, PartialEq)]
pub struct PatternCellEditorProps {
    pub thread_cell: (GridCell, IArray<Thread>),
    #[prop_or_default]
    pub onchange: Callback<(GridCell, IArray<Thread>)>,
    pub default_color: Rgb,
    pub on_close: Callback<MouseEvent>,
}

#[function_component(PatternCellEditor)]
pub fn pattern_cell_editor(props: &PatternCellEditorProps) -> Html {
    let PatternCellEditorProps {
        thread_cell,
        onchange,
        default_color,
        on_close,
    } = props;
    let slash_backwards_value =
        find_thread_type_hex_value(ThreadType::SlashBackwards, &thread_cell.1, *default_color);
    let slash_forwards_value =
        find_thread_type_hex_value(ThreadType::SlashForwards, &thread_cell.1, *default_color);
    let border_top_value =
        find_thread_type_hex_value(ThreadType::BorderTop, &thread_cell.1, *default_color);
    let border_right_value =
        find_thread_type_hex_value(ThreadType::BorderRight, &thread_cell.1, *default_color);
    let border_bottom_value =
        find_thread_type_hex_value(ThreadType::BorderBottom, &thread_cell.1, *default_color);
    let border_left_value =
        find_thread_type_hex_value(ThreadType::BorderLeft, &thread_cell.1, *default_color);

    let on_slash_backwards_change =
        create_on_change_event(ThreadType::SlashBackwards, thread_cell.clone(), onchange);
    let on_slash_forwards_change =
        create_on_change_event(ThreadType::SlashForwards, thread_cell.clone(), onchange);
    let on_border_top_change =
        create_on_change_event(ThreadType::BorderTop, thread_cell.clone(), onchange);
    let on_border_right_change =
        create_on_change_event(ThreadType::BorderRight, thread_cell.clone(), onchange);
    let on_border_bottom_change =
        create_on_change_event(ThreadType::BorderBottom, thread_cell.clone(), onchange);
    let on_border_left_change =
        create_on_change_event(ThreadType::BorderLeft, thread_cell.clone(), onchange);

    html! {<div>
        {"Cell editor"}
        <button onclick={on_close}>{ "Close" }</button>
        <div>
            <label for="slash_forwards">
                {"Slash Forwards"}
                <input type="color" name="slash_forwards" value={slash_forwards_value.clone()} onchange={on_slash_forwards_change} />
            </label>
        </div>
        <div>
            <label for="slash_backwards">
                {"Slash Backwards"}
                <input type="color" name="slash_backwards" value={slash_backwards_value.clone()} onchange={on_slash_backwards_change} />
            </label>
        </div>
        <div>
            <label for="border_top">
                {"Border Top"}
                <input type="color" name="border_top" value={border_top_value.clone()} onchange={on_border_top_change} />
            </label>
        </div>
        <div>
            <label for="border_right">
                {"Border Right"}
                <input type="color" name="border_right" value={border_right_value.clone()} onchange={on_border_right_change} />
            </label>
        </div>
        <div>
            <label for="border_bottom">
                {"Border Bottom"}
                <input type="color" name="border_bottom" value={border_bottom_value.clone()} onchange={on_border_bottom_change} />
            </label>
        </div>
        <div>
            <label for="border_left">
                {"Border Left"}
                <input type="color" name="border_left" value={border_left_value.clone()} onchange={on_border_left_change} />
            </label>
        </div>
    </div>}
}

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use html::ImplicitClone;
use implicit_clone::sync::{IArray, IMap};
use indexmap::IndexMap;
use log::{error, info};
use web_sys::{HtmlInputElement, MouseEvent};
use yew::prelude::*;

use crate::components::pattern::grid::PatternGrid;
use crate::components::pattern::{
    GridCell, GridSize, GridType, RawGridType, Rgb, Thread, ThreadType,
};
use crate::contexts::history::HistoryContext;

use super::super::components::pattern_cell_editor::PatternCellEditor;
use super::super::components::toolbar::Toolbar;
use super::super::PatternEditorFeature;

const DEFAULT_COLOR: Rgb = Rgb { r: 255, g: 0, b: 0 };

#[function_component(PatternEditor)]
pub fn pattern_editor() -> Html {
    let history_context = use_context::<HistoryContext>().expect("No DocumentContext found!");
    info!("hist: {:?}", history_context);
    let mousedown_elements_handle: UseStateHandle<GridType> = use_state(IMap::default);
    let colnum_ref = use_node_ref();
    let rownum_ref = use_node_ref();
    let active_feature_handle = use_state(|| PatternEditorFeature::Brush);
    let is_mousedown_handle = use_state(|| false);
    let grid_size_handle: UseStateHandle<Option<GridSize>> = use_state(|| None);
    let thread_color_handle = use_state(|| DEFAULT_COLOR);
    let active_thread_cell_handle: UseStateHandle<(GridCell, IArray<Thread>)> =
        use_state(|| (GridCell(0, 0), IArray::default()));
    let selected_cells_handle: UseStateHandle<IArray<GridCell>> =
        use_state(|| IArray::from(vec![]));
    let grid_size = *grid_size_handle;
    let active_thread_cell = (*active_thread_cell_handle).clone();
    let mousedown_elements = (*mousedown_elements_handle).clone();
    let selected_cells = (*selected_cells_handle).clone();
    let thread_color = *thread_color_handle;
    let clear_handle = use_state(|| true);

    let handle_dimensions_submit = use_callback(
        (
            colnum_ref.clone(),
            rownum_ref.clone(),
            grid_size_handle.clone(),
        ),
        |e: SubmitEvent, (colnum_ref, rownum_ref, grid_size_handle)| {
            e.prevent_default();
            if let (Some(col_input), Some(row_input)) = (
                colnum_ref.cast::<HtmlInputElement>(),
                rownum_ref.cast::<HtmlInputElement>(),
            ) {
                if let (Ok(col_num), Ok(row_num)) = (
                    col_input.value().parse::<usize>(),
                    row_input.value().parse::<usize>(),
                ) {
                    grid_size_handle.set(Some(GridSize(col_num, row_num)));
                } else {
                    error!("Unable to convert inputs values to usize");
                }
            } else {
                error!("Unable to find input elements");
            }
        },
    );
    let handle_thread_change = use_callback(
        (
            active_thread_cell_handle.clone(),
            history_context.clone(),
            mousedown_elements_handle.clone(),
        ),
        |(cell_key, updated_threads): (GridCell, IArray<Thread>),
         (active_thread_cell_handle, history_context, mousedown_elements_handle)| {
            let mut grid: RawGridType = history_context.get_grid();
            let mut updated_mousedown_elements = HashMap::new();
            if let Some(cell_value) = grid.get(&cell_key) {
                let previous_threads = cell_value;
                let mut new_threads = updated_threads.clone().to_vec();

                // Don't add previous Thread if updated ThreadType exists
                for thread in previous_threads.iter() {
                    if !new_threads
                        .iter()
                        .any(|Thread { thread_type, .. }| *thread_type == thread.thread_type)
                    {
                        new_threads.push(thread);
                    }
                }

                let updated_cell_tuple = (cell_key, IArray::from(new_threads));

                updated_mousedown_elements.insert(updated_cell_tuple.0, updated_cell_tuple.1);
            };

            {
                if grid.contains_key(&cell_key) {
                    let updated_cell_tuple = (cell_key, updated_threads.clone());
                    grid.insert(updated_cell_tuple.0, updated_cell_tuple.1.clone());

                    updated_mousedown_elements.insert(updated_cell_tuple.0, updated_cell_tuple.1);
                };
            };

            // There should only be 1 item in this iterator so .last() is always the only item
            if let Some((k, v)) = updated_mousedown_elements.iter().last() {
                active_thread_cell_handle.set((*k, v.clone()));
            }

            {
                let history_local = history_context.history;
                history_local.with(|history| {
                    history.borrow_mut().push(grid.clone());
                });
            };
            mousedown_elements_handle.set(hashmap_to_imap(updated_mousedown_elements));
        },
    );

    let handle_mousedown = use_callback(
        (
            (*active_feature_handle).clone(),
            active_thread_cell_handle.clone(),
            history_context.clone(),
            grid_size,
            is_mousedown_handle.clone(),
            mousedown_elements_handle.clone(),
            selected_cells_handle.clone(),
            thread_color,
        ),
        |cell_key: GridCell,
         (
            active_feature,
            active_thread_cell_handle,
            history_context,
            grid_size,
            is_mousedown_handle,
            mousedown_elements_handle,
            selected_cells_handle,
            thread_color,
        )| {
            let GridCell(col_index, row_index) = cell_key;
            match active_feature {
                PatternEditorFeature::Brush => {
                    let mut mousedown_elements: HashMap<GridCell, IArray<Thread>> = HashMap::new();

                    mousedown_elements.insert(
                        GridCell(col_index, row_index),
                        IArray::from(vec![
                            Thread::new(*thread_color, ThreadType::SlashForwards),
                            Thread::new(*thread_color, ThreadType::SlashBackwards),
                        ]),
                    );

                    info!("mousedownel: {:?}", mousedown_elements);

                    is_mousedown_handle.set(true);
                    mousedown_elements_handle.set(hashmap_to_imap(mousedown_elements));
                }
                PatternEditorFeature::Fill => {
                    let mut mousedown_elements: HashMap<GridCell, IArray<Thread>> = HashMap::new();
                    let GridSize(col_count, row_count) =
                        grid_size.unwrap_or(GridSize(0_usize, 0_usize));

                    for row_index in 0..row_count {
                        for col_index in 0..col_count {
                            mousedown_elements.insert(
                                GridCell(col_index, row_index),
                                IArray::from(vec![
                                    Thread::new(*thread_color, ThreadType::SlashForwards),
                                    Thread::new(*thread_color, ThreadType::SlashBackwards),
                                ]),
                            );
                        }
                    }

                    is_mousedown_handle.set(true);
                    mousedown_elements_handle.set(hashmap_to_imap(mousedown_elements));
                }
                PatternEditorFeature::Pointer => {
                    let grid: RawGridType = history_context.get_grid();
                    let selected_cells = (*selected_cells_handle).clone();
                    let GridSize(col_count, row_count) =
                        grid_size.unwrap_or(GridSize(0_usize, 0_usize));

                    if col_count > col_index && row_count > row_index {
                        if let Some(cell_value) = grid.get(&cell_key) {
                            active_thread_cell_handle.set((cell_key, cell_value.clone()));
                        } else {
                            active_thread_cell_handle
                                .set((GridCell(col_index, row_index), IArray::default()));
                        }
                    }

                    if let Some(selected_cell) = selected_cells.first() {
                        // Delesect if a cell is clicked twice
                        if selected_cells.len() == 1
                            && *selected_cell == GridCell(col_index, row_index)
                        {
                            selected_cells_handle.set(IArray::from(vec![]));
                        } else {
                            selected_cells_handle
                                .set(IArray::from(vec![GridCell(col_index, row_index)]));
                        }
                    } else {
                        selected_cells_handle
                            .set(IArray::from(vec![GridCell(col_index, row_index)]));
                    }
                }
            };
        },
    );

    let handle_mousemove = use_callback(
        (
            (*active_feature_handle).clone(),
            *is_mousedown_handle,
            mousedown_elements_handle.clone(),
            thread_color,
        ),
        |cell_key: GridCell,
         (active_feature, is_mousedown, mousedown_elements_handle, thread_color)| {
            let GridCell(col_index, row_index) = cell_key;
            match active_feature {
                PatternEditorFeature::Brush => {
                    if *is_mousedown {
                        let mousedown_elements = (*mousedown_elements_handle).clone();

                        if !mousedown_elements.contains_key(&cell_key) {
                            let mut mousedown_elements: HashMap<GridCell, IArray<Thread>> =
                                mousedown_elements.iter().collect();

                            mousedown_elements.insert(
                                GridCell(col_index, row_index),
                                IArray::from(vec![
                                    Thread::new(*thread_color, ThreadType::SlashForwards),
                                    Thread::new(*thread_color, ThreadType::SlashBackwards),
                                ]),
                            );

                            mousedown_elements_handle.set(hashmap_to_imap(mousedown_elements));
                        };
                    }
                }
                PatternEditorFeature::Fill => {}
                PatternEditorFeature::Pointer => {}
            };
        },
    );

    let handle_mouseup = use_callback(
        (
            (*active_feature_handle).clone(),
            history_context.clone(),
            is_mousedown_handle.clone(),
            mousedown_elements_handle.clone(),
        ),
        |_: MouseEvent,
         (
            active_feature,
            history_context,
            is_mousedown_handle,
            mousedown_elements_handle,
        )| {
            if **is_mousedown_handle {
                match active_feature {
                    PatternEditorFeature::Brush | PatternEditorFeature::Fill => {
                        let mousedown_elements: HashMap<GridCell, IArray<Thread>> =
                            (*mousedown_elements_handle).iter().collect();

                        {
                            history_context.history.with(|history| {
                                let mut history = history.borrow_mut();
                                history.push(mousedown_elements.clone());
                            });
                        };

                        mousedown_elements_handle.set(IMap::default());
                        is_mousedown_handle.set(false);
                    }
                    PatternEditorFeature::Pointer => {}
                };
            }
        },
    );

    let handle_undo = use_callback(
        (
            history_context.clone(),
            mousedown_elements_handle.clone(),
            clear_handle.clone(),
        ),
        |_: MouseEvent, (history_context, mousedown_elements_handle, clear_handle)| {
            let (history_len, second_last) = history_context.history.with(move |local_history| {
                let second_last = {
                    let local_history = local_history.borrow();
                    local_history.get(local_history.len() - 2).cloned()
                };
                let mut local_history = local_history.borrow_mut();

                local_history.pop();

                (local_history.len(), second_last)
            });

            if history_len >= 2 {
                if let Some(second_last) = second_last {
                    mousedown_elements_handle.set(hashmap_to_imap(second_last.clone()));
                    clear_handle.set(true);
                }
            } else {
                mousedown_elements_handle.set(IMap::default());
                clear_handle.set(true);
            }
        },
    );

    let handle_render = use_callback(clear_handle.clone(), |(), clear_handle| {
        clear_handle.set(false);
    });

    let handle_set_feature: Callback<PatternEditorFeature> = use_callback(
        (active_feature_handle.clone(), selected_cells_handle.clone()),
        move |feature: PatternEditorFeature, (active_feature_handle, selected_cells_handle)| {
            // Deselect all cells
            if feature != PatternEditorFeature::Pointer {
                selected_cells_handle.set(IArray::from(vec![]));
            }

            active_feature_handle.set(feature);
        },
    );

    let handle_on_color_change = use_callback(
        thread_color_handle.clone(),
        move |hex_value: String, thread_color_handle| {
            if let Ok(color) = Rgb::from_hex(hex_value) {
                thread_color_handle.set(color);
            }
        },
    );

    let handle_cell_editor_close = use_callback(
        selected_cells_handle.clone(),
        |_: MouseEvent, selected_cells_handle| {
            selected_cells_handle.set(IArray::from(vec![]));
        },
    );

    info!("render");
    html! {
        <div onmouseup={handle_mouseup}>
            <form onsubmit={handle_dimensions_submit}>
                <label>
                    { "Rows" }
                    <input ref={rownum_ref} type="text" name="rows" />
                </label>
                <label>
                    { "Cols" }
                    <input ref={colnum_ref} type="text" name="cols" />
                </label>
                <div>
                    <button type="submit">{ "Save input" }</button>
                </div>
            </form>

            if let Some(GridSize(col_count, row_count)) = grid_size {
                <Toolbar
                    default_color={DEFAULT_COLOR}
                    on_color_change={handle_on_color_change}
                    on_feature_change={handle_set_feature}
                    on_undo={handle_undo}
                />

                <h3>{ "This is a pattern editor" }</h3>

                <PatternGrid
                    onrender={handle_render}
                    onmousedown={handle_mousedown}
                    onmousemove={handle_mousemove}
                    selected_cells={selected_cells.clone()}
                    overwrite={mousedown_elements}
                    cols={col_count}
                    rows={row_count}
                    default_color={Rgb { r: 204, g: 204, b: 204 }}
                    clear={*clear_handle}
                />

                if !selected_cells.is_empty() {
                    {"PatternCellEditor"}
                    <PatternCellEditor on_close={handle_cell_editor_close} default_color={DEFAULT_COLOR} onchange={handle_thread_change} thread_cell={active_thread_cell} />
                }
            }
        </div>
    }
}

pub fn hashmap_to_imap<K, V>(hash_map: HashMap<K, V>) -> IMap<K, V>
where
    K: Eq + Hash + ImplicitClone + 'static,
    V: PartialEq + ImplicitClone + 'static,
{
    let index_map: IndexMap<K, V> = hash_map.into_iter().collect();
    IMap::Rc(Arc::new(index_map))
}

use std::collections::HashMap;

use implicit_clone::sync::{IArray, IMap};
use yew::prelude::*;

use crate::components::header::Header;
use crate::components::pattern::grid::PatternGrid;
use crate::components::pattern::{GridCell, GridType, Rgb, Thread, ThreadType};

const DEFAULT_COLOR: Rgb = Rgb {
    r: 204,
    g: 204,
    b: 204,
};

pub fn pattern_detail(id: String) -> Html {
    let threads = IArray::from(vec![
        Thread {
            color: Rgb { r: 255, g: 0, b: 0 },
            thread_type: ThreadType::SlashForwards,
        },
        Thread {
            color: Rgb { r: 0, g: 255, b: 0 },
            thread_type: ThreadType::BorderTop,
        },
        Thread {
            color: Rgb { r: 0, g: 255, b: 0 },
            thread_type: ThreadType::SlashBackwards,
        },
    ]);

    let mut thread_cells: HashMap<GridCell, IArray<Thread>> = HashMap::default();
    thread_cells.insert(GridCell(0, 0), threads.clone());
    thread_cells.insert(GridCell(0, 1), threads.clone());
    thread_cells.insert(GridCell(1, 0), threads.clone());
    thread_cells.insert(GridCell(1, 1), threads.clone());

    let thread_cells: GridType = thread_cells.iter().map(|(&k, v)| (k, v.clone())).collect();

    html! {<>
        <Header />
        <h1>{ format!("Pattern Detail: {}", id) }</h1>
        <PatternGrid default_color={DEFAULT_COLOR} overwrite={thread_cells} clear={true} rows={3} cols={3} />
    </>}
}

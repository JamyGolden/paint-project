use implicit_clone::sync::IArray;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;
use web_sys::{CanvasRenderingContext2d, DomRect, HtmlCanvasElement};
use yew::prelude::*;

use super::{GridCell, GridType, Rgb, Thread, ThreadType};
use crate::components::canvas::{Canvas, WithRender};

#[derive(Properties, PartialEq)]
pub struct PatternGridProps {
    #[prop_or_default]
    pub onrender: Callback<()>,
    #[prop_or_default]
    pub onmousedown: Callback<GridCell>,
    #[prop_or_default]
    pub onmousemove: Callback<GridCell>,
    #[prop_or_default]
    pub overwrite: GridType,
    #[prop_or_default]
    pub selected_cells: IArray<GridCell>,
    pub cols: usize,
    pub rows: usize,
    pub default_color: Rgb,
    pub clear: bool,
}

#[function_component(PatternGrid)]
pub fn pattern_grid(props: &PatternGridProps) -> Html {
    let PatternGridProps {
        cols,
        onrender,
        onmousedown,
        onmousemove,
        overwrite,
        rows,
        selected_cells,
        default_color,
        clear,
    } = props;
    // let mut is_first_render_ref = use_mut_ref(|| true);
    // let offscreen_canvas_ref = use_state(|| create_offscreen_canvas());

    // let grid: Rc<IMap<(usize, usize), IArray<Thread>>> = use_memo(
    //     (thread_cells.clone(), overwrite.clone()),
    //     |(thread_cells, overwrite)| {
    //         let mut grid: HashMap<(usize, usize), IArray<Thread>> = HashMap::default();
    //         for row in 0..*rows {
    //             for col in 0..*cols {
    //                 if let Some((k, v)) = overwrite.iter().find(|(k, _)| k.0 == col && k.1 == row) {
    //                     grid.insert(k, v);
    //                 } else if let Some((k, v)) =
    //                     thread_cells.iter().find(|(k, _)| k.0 == col && k.1 == row)
    //                 {
    //                     grid.insert(k, v);
    //                 }
    //             }
    //         }

    //         grid.iter()
    //             .map(|(&k, v)| (k, v.clone()))
    //             .collect::<IMap<(usize, usize), IArray<Thread>>>()
    //     },
    // );
    // {
    //     let onload = onload.clone();

    //     use_effect_with((), move |_| {
    //         onload.emit(());
    //     })
    // };
    let cell_width = 20.0;
    let cell_height = 20.0;
    let render = use_memo(
        (
            overwrite.clone(),
            selected_cells.clone(),
            *clear,
            onrender.clone(),
        ),
        |(overwrite, selected_cells, clear, onrender)| Render {
            redraw: (*clear, onrender.clone()),
            size: GridCell(*cols, *rows),
            default_color: *default_color,
            thread_cells: overwrite.clone(),
            selected_cells: selected_cells.clone(),
            cell_width,
            cell_height,
            border_width: 1.0,
        },
    );
    let node_ref = NodeRef::default();
    let handle_mousedown = use_callback(
        (node_ref.clone(), render.clone(), onmousedown.clone()),
        |event: MouseEvent, (node_ref, render, onmousedown)| {
            if let Some(canvas) = node_ref.cast::<HtmlCanvasElement>() {
                let client_x = event.client_x();
                let client_y = event.client_y();
                let rect = canvas.get_bounding_client_rect();

                onmousedown.emit(get_thread_cell((client_x, client_y), rect, render));
            }
        },
    );
    let handle_mousemove = use_callback(
        (node_ref.clone(), render.clone(), onmousemove.clone()),
        |event: MouseEvent, (node_ref, render, onmousemove)| {
            if let Some(canvas) = node_ref.cast::<HtmlCanvasElement>() {
                let client_x = event.client_x();
                let client_y = event.client_y();
                let rect = canvas.get_bounding_client_rect();

                onmousemove.emit(get_thread_cell((client_x, client_y), rect, render));
            }
        },
    );

    // let message_count = use_mut_ref(|| 0);
    // let message_count = use_mut_ref(|| 0);

    // info!("isfirstrender: {:?}", *is_first_render_handle);
    // let meh = is_first_render_ref::clone();
    // let is_first_render_ref = use_mut_ref(|| true);
    // if (*is_first_render_ref).borrow_mut() == true {
    //     *is_first_render_ref.borrow_mut() = false;
    // };
    // let is_first_render_ref = use_mut_ref(|| true);
    // let is_first_render_ref = use_mut_ref(|| Rc::new(RefCell::new(true)));
    // if *is_first_render_ref.borrow() {
    //     is_first_render_ref.borrow_mut().replace(false);
    // }
    // if *message_count.borrow_mut() > 3 {
    //     window.alert_with_message("Message limit reached");
    // } else {
    //     *message_count.borrow_mut() += 1;
    //     window.alert_with_message("Message sent");
    // }
    // {
    //     let is_first_render_handle = is_first_render_handle;
    //     use_effect(move || if is_first_render_handle.set(false));
    //     // use_effect(move || {
    //     //     // Make a call to DOM API after component is rendered
    //     //     gloo_utils::document().set_title(&format!("You clicked {} times", *counter));

    //     //     // Perform the cleanup
    //     //     || gloo_utils::document().set_title("You clicked 0 times")
    //     // });
    // };

    html! {<>
    <Canvas<CanvasRenderingContext2d, Render>
        style=""
        width={rows * cell_width as usize}
        height={cols * cell_height as usize}
        node_ref={node_ref}
        render={Box::new((*render).clone())}
        onmousedown={handle_mousedown}
        onmousemove={handle_mousemove}
    >
        <div>{"The browser is not supported."}</div>
    </Canvas<CanvasRenderingContext2d, Render>>
    </>}
}

struct GridContext {
    cell_width: f64,
    cell_height: f64,
    border_width: f64,
}

impl GridContext {
    pub fn new(cell_width: f64, cell_height: f64, border_width: f64) -> Self {
        GridContext {
            cell_width,
            cell_height,
            border_width,
        }
    }
}

fn draw_grid(
    GridCell(cols, rows): GridCell,
    color: Rgb,
    ctx: &CanvasRenderingContext2d,
    grid_ctx: &GridContext,
) {
    let grid_width = cols as f64 * grid_ctx.cell_width;
    let grid_height = rows as f64 * grid_ctx.cell_height;
    ctx.clear_rect(0.0, 0.0, grid_width, grid_height);
    ctx.set_stroke_style(&JsValue::from_str(&color.as_hex()));

    for col in 0..cols {
        let cell_x = col as f64 * grid_ctx.cell_width;
        let cell_y = 0 as f64;
        ctx.set_line_width(grid_ctx.border_width);
        ctx.begin_path();
        ctx.move_to(cell_x, cell_y);
        ctx.line_to(grid_width, (cols - col) as f64 * grid_ctx.cell_height);
        ctx.stroke();
        ctx.fill();

        let cell_x = (cols - col) as f64 * grid_ctx.cell_width;
        let cell_y = grid_height;
        ctx.set_line_width(grid_ctx.border_width);
        ctx.begin_path();
        ctx.move_to(cell_x, cell_y);
        ctx.line_to(grid_width, (cols - col) as f64 * grid_ctx.cell_height);
        ctx.stroke();
        ctx.fill();
    }

    for row in 1..(rows + 1) {
        let cell_x = 0 as f64;
        let cell_y = row as f64 * grid_ctx.cell_width;
        ctx.set_line_width(grid_ctx.border_width);
        ctx.begin_path();
        ctx.move_to(cell_x, cell_y);
        ctx.line_to((rows - row) as f64 * grid_ctx.cell_width, grid_width);
        ctx.stroke();
        ctx.fill();

        let cell_x = 0_f64;
        let cell_y = row as f64 * grid_ctx.cell_height;
        ctx.set_line_width(grid_ctx.border_width);
        ctx.begin_path();
        ctx.move_to(cell_x, cell_y);
        ctx.line_to(row as f64 * grid_ctx.cell_width, 0 as f64);
        ctx.stroke();
        ctx.fill();
    }
}

fn draw_cell(
    grid_ctx: &GridContext,
    ctx: &CanvasRenderingContext2d,
    (GridCell(col_index, row_index), threads): (GridCell, IArray<Thread>),
    selected_cells: &IArray<GridCell>,
) {
    let is_selected = selected_cells
        .iter()
        .any(|t| t.0 == col_index && t.1 == row_index);
    let col_index = col_index as f64;
    let row_index = row_index as f64;
    let cell_x = col_index * grid_ctx.cell_width;
    let cell_y = row_index * grid_ctx.cell_height;

    ctx.set_line_width(grid_ctx.border_width);
    ctx.clear_rect(cell_x, cell_y, grid_ctx.cell_width, grid_ctx.cell_height);

    for thread in threads.iter() {
        match thread {
            Thread {
                thread_type: ThreadType::BorderTop,
                color,
            } => {
                // let hex_color = if is_selected {
                //     &JsValue::from_str("#000000")
                // } else {
                let hex_color = &JsValue::from_str(&color.as_hex());
                // };
                ctx.set_fill_style(hex_color);
                ctx.fill_rect(cell_x, cell_y, grid_ctx.cell_width, grid_ctx.border_width);
            }
            Thread {
                thread_type: ThreadType::BorderRight,
                color,
            } => {
                // let hex_color = if is_selected {
                //     &JsValue::from_str("#000000")
                // } else {
                let hex_color = &JsValue::from_str(&color.as_hex());
                // };
                let x = cell_x + grid_ctx.cell_width - grid_ctx.border_width;
                let y = cell_y;
                ctx.set_fill_style(hex_color);
                ctx.fill_rect(x, y, grid_ctx.border_width, grid_ctx.cell_width);
            }
            Thread {
                thread_type: ThreadType::BorderBottom,
                color,
            } => {
                // let hex_color = if is_selected {
                //     &JsValue::from_str("#000000")
                // } else {
                let hex_color = &JsValue::from_str(&color.as_hex());
                // };
                let x = cell_x;
                let y = cell_y + grid_ctx.cell_height - grid_ctx.border_width;
                ctx.set_fill_style(hex_color);
                ctx.fill_rect(x, y, grid_ctx.cell_width, grid_ctx.border_width);
            }
            Thread {
                thread_type: ThreadType::BorderLeft,
                color,
            } => {
                // let hex_color = if is_selected {
                //     &JsValue::from_str("#000000")
                // } else {
                let hex_color = &JsValue::from_str(&color.as_hex());
                // };
                ctx.set_fill_style(hex_color);
                ctx.fill_rect(cell_x, cell_y, grid_ctx.border_width, grid_ctx.cell_width);
            }
            Thread {
                thread_type: ThreadType::SlashForwards,
                color,
            } => {
                // let hex_color = if is_selected {
                //     &JsValue::from_str("#000000")
                // } else {
                let hex_color = &JsValue::from_str(&color.as_hex());
                // };
                ctx.set_stroke_style(hex_color);
                ctx.begin_path();
                ctx.move_to(
                    cell_x,
                    cell_y + grid_ctx.cell_height - grid_ctx.border_width,
                );
                ctx.line_to(cell_x + grid_ctx.cell_width, cell_y);
                ctx.stroke();
                ctx.fill();
            }
            Thread {
                thread_type: ThreadType::SlashBackwards,
                color,
            } => {
                // let hex_color = if is_selected {
                //     &JsValue::from_str("#000000")
                // } else {
                let hex_color = &JsValue::from_str(&color.as_hex());
                // };
                ctx.set_stroke_style(hex_color);
                ctx.begin_path();
                ctx.move_to(cell_x, cell_y);
                ctx.line_to(cell_x + grid_ctx.cell_width, cell_y + grid_ctx.cell_height);
                ctx.stroke();
                ctx.fill();
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Render {
    redraw: (bool, Callback<()>),
    size: GridCell,
    default_color: Rgb,
    thread_cells: GridType,
    selected_cells: IArray<GridCell>,
    cell_width: f64,
    cell_height: f64,
    border_width: f64,
}

impl WithRender for Render {
    fn render(self, canvas: &HtmlCanvasElement) {
        let grid_ctx = GridContext::new(self.cell_width, self.cell_height, self.border_width);
        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        let closure = Closure::wrap(Box::new(move || {
            if self.redraw.0 {
                draw_grid(self.size, self.default_color, &ctx, &grid_ctx);
            }

            for thread_cell in self.thread_cells.iter() {
                draw_cell(&grid_ctx, &ctx, thread_cell, &self.selected_cells);
            }
        }) as Box<dyn FnMut()>);

        window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .expect("not able to get raf");
        closure.forget(); // Prevents the closure from being cleaned up

        if self.redraw.0 {
            self.redraw.1.emit(());
        }
    }
}

fn get_thread_cell((client_x, client_y): (i32, i32), rect: DomRect, render: &Render) -> GridCell {
    let relative_x = (client_x as f64) - rect.x();
    let relative_y = (client_y as f64) - rect.y();

    let col_index = (relative_x / render.cell_width).floor() as usize;
    let row_index = (relative_y / render.cell_height).floor() as usize;

    GridCell(col_index, row_index)
}

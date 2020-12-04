use std::{ rc::Rc, cell::{Cell, RefCell} };
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let canvas_w = 640.0;
    let canvas_h = 480.0;
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(canvas_w as u32);
    canvas.set_height(canvas_h as u32);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let dragged = Rc::new(Cell::new(false));
    let w = 10.0; // width for point rect
    let id: Option<usize> = None; // index of dragged point 
    let id = Rc::new(Cell::new(id));
    let points = vec![[10.0, 10.0],[100.0, 30.0],[200.0, 30.0],[300.0, 10.0]];
    draw_bezier(points.clone(), w, context.clone());
    let points = Rc::new(RefCell::new(points));
    {
        let context = context.clone();
        let dragged = dragged.clone();
        let id = id.clone();
        let points = points.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            let mouse_x = event.offset_x() as f64;
            let mouse_y = event.offset_y() as f64;
            for (i, p) in points.borrow().iter().enumerate() {
                if mouse_x - w/2.0 < p[0] && p[0] <  mouse_x + w/2.0 && mouse_y - w/2.0 < p[1] && p[1] < mouse_y + w/2.0 {
                    dragged.set(true);
                    id.set(Some(i));
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let dragged = dragged.clone();
        let id = id.clone();
        let points = points.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if dragged.get() {
                context.clear_rect(0.0, 0.0, canvas_w, canvas_h);
                let mouse_x = event.offset_x() as f64;
                let mouse_y = event.offset_y() as f64;
                if id.get() != None {
                    points.borrow_mut()[id.get().unwrap()] = [mouse_x, mouse_y];
                }
                draw_bezier(points.borrow().to_vec(), w, context.clone());
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let dragged = dragged.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            if dragged.get() {
                dragged.set(false);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

fn draw_bezier(points: Vec<[f64; 2]>, w: f64, context: Rc<web_sys::CanvasRenderingContext2d> ) {
    for p in points.iter() {
        context.fill_rect(p[0] - w/2.0, p[1] - w/2.0, w, w);
    }
    context.begin_path();
    context.move_to(points[0][0],points[0][1]);
    context.line_to(points[1][0],points[1][1]);
    context.stroke();
    context.begin_path();
    context.move_to(points[0][0],points[0][1]);
    context.bezier_curve_to(points[1][0],points[1][1],points[2][0],points[2][1],points[3][0],points[3][1]);
    context.stroke();
    context.begin_path();
    context.move_to(points[3][0],points[3][1]);
    context.line_to(points[2][0],points[2][1]);
    context.stroke();
}
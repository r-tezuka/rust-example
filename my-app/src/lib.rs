use std::{ rc::Rc, cell::{Cell, RefCell} };
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    const CANVAS_W: f64 = 640.0;
    const CANVAS_H: f64 = 480.0;
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(CANVAS_W as u32);
    canvas.set_height(CANVAS_H as u32);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    const W: f64 = 10.0; // width for point rect
    let id: Option<usize> = None; // index of dragged point 
    let id = Rc::new(Cell::new(id));
    let points = vec![Point{x: 10.0, y: 10.0},Point{x: 100.0, y: 30.0},Point{x: 200.0, y: 30.0},Point{x: 300.0, y: 10.0}];
    draw(points.clone(), W, context.clone());
    let points = Rc::new(RefCell::new(points));
    {
        let context = context.clone();
        let id = id.clone();
        let points = points.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            let mouse_x = event.offset_x() as f64;
            let mouse_y = event.offset_y() as f64;
            for (i, p) in points.borrow().iter().enumerate() {
                if mouse_x - W/2.0 < p.x && p.x <  mouse_x + W/2.0 && mouse_y - W/2.0 < p.y && p.y < mouse_y + W/2.0 {
                    id.set(Some(i));
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let id = id.clone();
        let points = points.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if id.get() != None {
                context.clear_rect(0.0, 0.0, CANVAS_W, CANVAS_H);
                let mouse_x = event.offset_x() as f64;
                let mouse_y = event.offset_y() as f64;
                points.borrow_mut()[id.get().unwrap()] = Point{x: mouse_x, y: mouse_y};
                draw(points.borrow().to_vec(), W, context.clone());
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            if id.get() != None {
                id.set(None);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

fn draw(points: Vec<Point>, w: f64, context: Rc<web_sys::CanvasRenderingContext2d> ) {
    //draw curve handles
    draw_handle(points[0].clone(), points[1].clone(), context.clone());
    draw_handle(points[3].clone(), points[2].clone(), context.clone());

    //draw points
    for p in points.iter() {
        context.fill_rect(p.x - w/2.0, p.y - w/2.0, w, w);
    }

    //draw bezier curve
    context.begin_path();
    context.move_to(points[0].x,points[0].y);
    context.bezier_curve_to(points[1].x,points[1].y,points[2].x,points[2].y,points[3].x,points[3].y);
    context.stroke();
}

fn draw_handle(from: Point, to: Point, context: Rc<web_sys::CanvasRenderingContext2d>) {
    context.begin_path();
    context.move_to(from.x, from.y);
    context.line_to(to.x, to.y);
    context.stroke();
}
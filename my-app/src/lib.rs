use std::{ rc::Rc, cell::{Cell, RefCell} };
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let dragged = Rc::new(Cell::new(false));
    let target = Rc::new(Cell::new([0.0, 0.0]));
    let point_width = 10.0;
    let w = Rc::new(point_width);
    let points = vec![[10.0, 10.0],[100.0, 10.0],[200.0, 10.0]];
    for p in points.iter() {
        context.fill_rect(p[0] - *w/2.0, p[1] - *w/2.0, *w, *w);
    }
    let points = Rc::new(RefCell::new(points));
    {
        let context = context.clone();
        let dragged = dragged.clone();
        let w = w.clone();
        let points = points.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            let mouse_x = event.offset_x() as f64;
            let mouse_y = event.offset_y() as f64;
            let mut id: Option<usize> = None;
            for (i, p) in points.borrow().iter().enumerate() {
                if mouse_x - *w/2.0 < p[0] && p[0] <  mouse_x + *w/2.0 && mouse_y - *w/2.0 < p[1] && p[1] < mouse_y + *w/2.0 {
                    dragged.set(true);
                    id = Some(i);
                }else{
                    context.fill_rect(p[0] - *w/2.0, p[1] - *w/2.0, *w, *w);
                }
            }
            if id != None {
                points.borrow_mut().remove(id.unwrap());
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let dragged = dragged.clone();
        let target = target.clone();
        let w = w.clone();
        let points = points.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if dragged.get() {
                context.clear_rect(0.0, 0.0, 640.0, 480.0);
                for p in points.borrow().iter() {
                    context.fill_rect(p[0] - *w/2.0, p[1] - *w/2.0, *w, *w);
                }
                let mouse_x = event.offset_x() as f64;
                let mouse_y = event.offset_y() as f64;
                target.set([mouse_x, mouse_y]);
                context.fill_rect(mouse_x - *w/2.0 , mouse_y - *w/2.0, *w, *w);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let dragged = dragged.clone();
        let points = points.clone();
        let target = target.clone();
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            if dragged.get() {
                points.borrow_mut().push(target.get());
                dragged.set(false);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
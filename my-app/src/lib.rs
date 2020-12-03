use std::{ rc::Rc, cell::Cell };
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

    let x = Rc::new(Cell::new(10.0));
    let y = Rc::new(Cell::new(10.0));
    let point_width = 10.0;
    let w = Rc::new(point_width);
    context.fill_rect(x.get() - *w/2.0, y.get() - *w/2.0, *w, *w);
    {
        let context = context.clone();
        let dragged = dragged.clone();
        let x = x.clone();
        let y = y.clone();
        let w = w.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            let mouse_x = event.offset_x() as f64;
            let mouse_y = event.offset_y() as f64;
            if mouse_x - *w/2.0 < x.get() && x.get() <  mouse_x + *w/2.0 && mouse_y - *w/2.0 < y.get() && y.get() < mouse_y + *w/2.0 {
                dragged.set(true);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let dragged = dragged.clone();
        let w = w.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if dragged.get() {
                context.clear_rect(0.0, 0.0, 640.0, 480.0);
                let mouse_x = event.offset_x() as f64;
                let mouse_y = event.offset_y() as f64;
                x.set(mouse_x);
                y.set(mouse_y);
                context.fill_rect(mouse_x - *w/2.0 , mouse_y - *w/2.0, *w, *w);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let dragged = dragged.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            dragged.set(false);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
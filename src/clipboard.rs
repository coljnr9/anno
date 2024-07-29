use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::CLIPBOARD_DATA;
use dioxus::signals::{Signal, Writable};
use web_sys::{
    wasm_bindgen::{closure::Closure, JsValue},
    window,
};

pub async fn use_clipboard() -> String {
    log::info!("In use_clipboard");
    let window = window().expect("Expected there to be a window");

    let clipboard = window.navigator().clipboard();

    let s = Rc::new(RefCell::new(String::new()));
    let s2 = Rc::clone(&s);

    let read_text = |s: Rc<RefCell<String>>| async move {
        if let Some(clipboard) = clipboard.clone() {
            let read_closure = Closure::wrap(Box::new(move |data: JsValue| {
                let d = data.as_string().expect("Error getting string");
                log::info!("In closure: {d}");
                *s.borrow_mut() = d;
            }) as Box<dyn FnMut(JsValue)>);
            let promise = clipboard.read_text().then(&read_closure);
            let res = wasm_bindgen_futures::JsFuture::from(promise).await;
            read_closure.forget();
        }
    };

    read_text(s).await;
    log::info!("About to return {:?}", s2);
    let v = match s2.try_borrow() {
        Ok(s) => s.to_owned(),
        Err(e) => format!("Clipboard data unavailable: {e}"),
    };
    v
}

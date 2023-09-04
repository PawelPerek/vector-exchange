use std::{rc::Rc, cell::RefCell, time::Duration};

use leptos::*;
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "create")]
    fn create_monaco(parent: &JsValue);

    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "onInput")]
    fn on_input(listener: &Closure<dyn Fn(String)>);
}

#[component]
pub fn Editor(cx: Scope, set_code: WriteSignal<String>) -> impl IntoView {
    let editor_parent = view!{ cx, 
        <div class="h-full w-full"></div>  
    };

    create_monaco(&editor_parent);

    let event_listener = Closure::wrap(Box::new(move |code: String| {
        set_code(code);
    }) as Box<dyn Fn(String)>);

    on_input(&event_listener);

    std::mem::forget(event_listener);

    editor_parent
}
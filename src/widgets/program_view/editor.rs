use eeric::prelude::*;
use leptos::*;
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "create")]
    fn create_monaco(parent: &JsValue);

    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "onInput")]
    fn on_input(listener: &Closure<dyn Fn(String)>);

    #[wasm_bindgen(js_namespace = monacoBridge)]
    fn enable();

    #[wasm_bindgen(js_namespace = monacoBridge)]
    fn disable();

    #[wasm_bindgen(js_namespace = monacoBridge, js_name = "highlightLine")]
    fn highlight_line(line: usize);
}

#[component]
pub fn Editor(cx: Scope, set_code: WriteSignal<String>) -> impl IntoView {
    // Create
    
    let editor_parent = view! { cx,
        <div class="h-full w-full"></div>
    };

    create_monaco(&editor_parent);

    // Listen to code change

    let event_listener = Closure::wrap(Box::new(move |code: String| {
        set_code(code);
    }) as Box<dyn Fn(String)>);

    on_input(&event_listener);

    std::mem::forget(event_listener);

    // Update writability after compilation

    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);

    create_effect(cx, move |_| {
        match core() {
            Some(_) => disable(),
            None => enable(),
        }
    });


    editor_parent
}

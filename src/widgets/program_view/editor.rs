use std::{rc::Rc, cell::RefCell};

use leptos::*;
use monaco::api::*;
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen(module="/src/js/monaco.js")]
extern "C" {
    #[wasm_bindgen(js_name = "fitToParent")]
    fn fit_to_parent(parent: &JsValue, monaco: &JsValue);
}

#[component]
pub fn Editor(cx: Scope, set_code: WriteSignal<String>) -> impl IntoView {
    let editor_parent = view!{ cx, 
        <div class="h-full w-full"></div>  
    };

    let initial_value = "addi x1, x0, 1\nloop:\n\tadd x1, x1, x1\n\tbeq x0, x0, loop\n";

    let editor = CodeEditor::create(
        &editor_parent, 
        Some(CodeEditorOptions::default()
            .with_builtin_theme(monaco::sys::editor::BuiltinTheme::Vs)
            .with_value(initial_value.to_owned())
        )
    );

    fit_to_parent(&editor_parent, editor.as_ref());

    set_code(initial_value.to_owned());

    let model = Rc::new(RefCell::new(editor.get_model().unwrap()));

    let cloned_model = model.clone();

    let dispose = model.borrow().on_did_change_content(move |_| {
        let code = cloned_model.borrow().get_value();

        set_code(code);
    });

    std::mem::forget(editor);
    std::mem::forget(dispose);

    editor_parent
}
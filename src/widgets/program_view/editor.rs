use std::{rc::Rc, cell::RefCell};

use leptos::*;
use monaco::{api::*, sys::editor::IDimension};

#[component]
pub fn Editor(cx: Scope, set_code: WriteSignal<String>) -> impl IntoView {
    let (element, _) = create_signal(cx, view!{ cx, 
        <div class="h-full w-full"></div>  
    });

    let height = window().inner_height().unwrap().as_f64().unwrap() - 178.0;
    let width = window().inner_width().unwrap().as_f64().unwrap() / 2.0;

    let initial_value = "addi x1, x0, 1\nloop:\n\tadd x1, x1, x1\n\tbeq x0, x0, loop\n";

    let editor = CodeEditor::create(
        &element(), 
        Some(CodeEditorOptions::default()
            .with_builtin_theme(monaco::sys::editor::BuiltinTheme::Vs)
            .with_dimension(IDimension::new(width, height))
            .with_value(initial_value.to_owned())
        )
    );

    set_code(initial_value.to_owned());

    let model = Rc::new(RefCell::new(editor.get_model().unwrap()));

    let cloned_model = model.clone();

    let dispose = model.borrow().on_did_change_content(move |_| {
        let code = cloned_model.borrow().get_value();
        
        log!("{:?}", code);
        set_code(code);
    });

    std::mem::forget(editor);
    std::mem::forget(dispose);

    element
}
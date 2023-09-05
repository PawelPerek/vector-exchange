use eeric::prelude::*;
use leptos::*;

#[component]
pub fn MemoryView (
    cx: Scope
) -> impl IntoView {
    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);
    let _ = create_read_slice(cx, core, |state| state.as_ref().map(|machine| machine.memory.snapshot()).unwrap_or_default());

    view! {cx,
        <>
            "todo :)"
        </>
    }
}
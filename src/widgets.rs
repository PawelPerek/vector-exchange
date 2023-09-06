mod program_view;
mod registers_view;
mod top_bar;

use eeric::prelude::*;
use leptos::*;

use program_view::ProgramView;
use registers_view::RegistersView;
use top_bar::TopBar;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, create_rw_signal(cx, None::<RvCore>));
    provide_context(cx, create_rw_signal(cx, VLEN::V128));

    // let core = create_rw_signal(cx, None::<RvCore>);

    // let reg_snapshot = move || core().map(|core| core.registers.snapshot()).unwrap_or_default();
    // let vu_snapshot = move || core().map(|core| core.vec_engine.snapshot()).unwrap_or_default();
    // let mem_snapshot = move || core().map(|core| core.memory.snapshot()).unwrap_or_default();

    view! {
        cx,
        <div
            style=r#"
            grid-template:
                "top top" 4rem
                "pro reg" calc(100vh - 4rem) / 40vw 60vw;
            "#
         class="grid h-screen overflow-y-hidden">
            <TopBar />
            <ProgramView />
            <RegistersView />
        </div>
    }
}

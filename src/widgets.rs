mod top_bar;
mod program_view;
mod registers_view;

use leptos::*;
use eeric::prelude::*;

use top_bar::TopBar;
use program_view::ProgramView;
use registers_view::RegistersView;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let core = create_rw_signal(cx, None::<RvCore>);

    let reg_snapshot = move || core().map(|core| core.registers.snapshot()).unwrap_or_default();
    let vu_snapshot = move || core().map(|core| core.vec_engine.snapshot()).unwrap_or_default();
    let mem_snapshot = move || core().map(|core| core.memory.snapshot()).unwrap_or_default();

    view! {
        cx,
        <div
            style=r#"
            grid-template:
                "top top" 4rem
                "pro reg" 1fr / 50vw 1fr;
            "#
         class="grid h-screen overflow-y-hidden">
            <TopBar />
            <ProgramView machine=core />
            <RegistersView 
                reg_snapshot=reg_snapshot
                vu_snapshot=vu_snapshot
                mem_snapshot=mem_snapshot
            />
        </div>
    }
}
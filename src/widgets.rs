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
    let snapshot_signal = create_rw_signal(cx, None::<RegistersSnapshot>);

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
            <ProgramView snapshot=snapshot_signal/>
            <RegistersView registers_snapshot=snapshot_signal.read_only()/>
        </div>
    }
}
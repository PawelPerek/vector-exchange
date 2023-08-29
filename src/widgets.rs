mod program_view;
mod registers_view;

use leptos::*;
use eeric::prelude::*;
use program_view::ProgramView;
use registers_view::RegistersView;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let snapshot_signal = create_rw_signal(cx, None::<RegistersSnapshot>);

    view! {
        cx,
        <div class="flex h-screen">
            <ProgramView snapshot=snapshot_signal/>
            <RegistersView registers_snapshot=snapshot_signal.read_only()/>
        </div>
    }
}
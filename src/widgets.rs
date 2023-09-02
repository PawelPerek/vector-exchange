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
    let snapshot = move || core.get().map(|machine| machine.registers_snapshot()).unwrap_or_default();

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
            <RegistersView snapshot=snapshot />
        </div>
    }
}

#[component]
fn Irrelevant(cx: Scope, value: ReadSignal<u32>, set_single: WriteSignal<u32>) -> impl IntoView {
    view! {cx, <div on:click=move |_| {
        set_single(value() * 2);
        log!("{}", value());
    }>{value}</div>}
}
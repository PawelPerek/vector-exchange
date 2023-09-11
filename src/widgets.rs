mod program_view;
mod registers_view;
mod top_bar;

use eeric::prelude::*;
use leptos::*;

use program_view::ProgramView;
use registers_view::RegistersView;
use top_bar::TopBar;

#[derive(Clone, Copy)]
pub enum Highlight {
    Off,
    On(usize)
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, create_rw_signal(cx, None::<RvCore>));
    provide_context(cx, create_rw_signal(cx, Vlen::V128));
    provide_context(cx, create_rw_signal(cx, Highlight::Off));

    view! { cx,
        <div
            style=r#"
            grid-template:
            "top top" 4rem
            "pro reg" calc(100vh - 4rem) / 40vw 60vw;
            "#
            class="grid h-screen overflow-y-hidden"
        >
            <TopBar/>
            <ProgramView/>
            <RegistersView/>
        </div>
    }
}

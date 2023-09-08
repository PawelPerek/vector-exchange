use eeric::prelude::*;
use leptos::*;

use crate::widgets::registers_view::vector_view::SEWType;

use super::{FrontEndLMUL, FrontEndSEW, FrontEndVLEN};

#[component]
pub fn VectorConfig(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);
    let vec_engine = create_read_slice(cx, core, |state| {
        state
            .as_ref()
            .map(|machine| machine.vec_engine.snapshot())
            .unwrap_or_default()
    });

    view! {
    cx,
        <div
            style=r#"
                grid-template:
                    "vlen vlen"
                    "sew  lmul";
            "#
            class="w-2/3 max-w-xl h-40 grid rounded divide-x divide-y bg-white"
        >
            <div style="grid-area: vlen" class="flex justify-evenly items-center">
                <span class="font-bold">Machine VLEN</span>
                <div class="flex divide-x shadow rounded cursor-pointer">
                    <VlenSelector vlen={FrontEndVLEN(VLEN::V64)} />
                    <VlenSelector vlen={FrontEndVLEN(VLEN::V128)} />
                    <VlenSelector vlen={FrontEndVLEN(VLEN::V256)} />
                    // Impossible to style :/
                    // <VlenSelector vlen={FrontEndVLEN(VLEN::V512)} current_vlen={selected_vlen}/>
                </div>
            </div>
            <div style="grid-area: sew" class="flex flex-col justify-center items-center font-bold">
                Machine SEW = {move || FrontEndSEW::Exact((vec_engine().sew, SEWType::Int)).to_string()}
            </div>
            <div style="grid-area: lmul" class="flex justify-center items-center font-bold">
                Machine LMUL = {move || FrontEndLMUL::Exact(vec_engine().lmul).to_string()}
            </div>
        </div>
    }
}

#[component]
pub fn VlenSelector(cx: Scope, vlen: FrontEndVLEN) -> impl IntoView {
    let selected_vlen = expect_context::<RwSignal<VLEN>>(cx);
    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);
    let is_started = create_read_slice(cx, core, |state| state.is_some());

    view! {
    cx,
        <div
            class="px-4 py-2 select-none"
            class=("font-bold", move || FrontEndVLEN(selected_vlen()) == vlen)
            class=("bg-gray-100", move || FrontEndVLEN(selected_vlen()) == vlen)
            class=("hover:bg-gray-100", move || !is_started())
            prop:disabled=is_started
            on:click=move |_| {
                selected_vlen.set(*vlen);
            }
        >
            {vlen.to_string()}
        </div>
    }
}

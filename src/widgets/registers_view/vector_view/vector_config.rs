use eeric::prelude::*;
use leptos::*;

use crate::widgets::{registers_view::vector_view::SEWType, MachineState};

use super::{FrontEndLMUL, FrontEndSEW, FrontEndVLEN};

#[component]
pub fn VectorConfig(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<MachineState>>(cx);
    let vec_engine = create_read_slice(cx, core, |state| {
        state
            .read_core()
            .map(|machine| machine.vec_engine.snapshot())
            .unwrap_or_default()
    });

    view! { cx,
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
                <div class="flex divide-x shadow rounded">
                    <VlenSelector vlen=FrontEndVLEN(Vlen::V64)/>
                    <VlenSelector vlen=FrontEndVLEN(Vlen::V128)/>
                    <VlenSelector vlen=FrontEndVLEN(Vlen::V256)/>
                    <VlenSelector vlen=FrontEndVLEN(Vlen::V512)/>
                </div>
            </div>
            <div style="grid-area: sew" class="flex flex-col justify-center items-center font-bold">
                Machine SEW =
                {move || FrontEndSEW::Exact((vec_engine().sew, SEWType::Int)).to_string()}
            </div>
            <div style="grid-area: lmul" class="flex justify-center items-center font-bold">
                Machine LMUL =
                {move || FrontEndLMUL(vec_engine().lmul).to_string()}
            </div>
        </div>
    }
}

#[component]
pub fn VlenSelector(cx: Scope, vlen: FrontEndVLEN) -> impl IntoView {
    let selected_vlen = expect_context::<RwSignal<Vlen>>(cx);
    let core = expect_context::<RwSignal<MachineState>>(cx);
    let is_started = create_read_slice(cx, core, |state| state.is_on());

    view! { cx,
        <div
            class="px-4 py-2 select-none"
            class=("font-bold", move || FrontEndVLEN(selected_vlen()) == vlen)
            class=("bg-gray-100", move || FrontEndVLEN(selected_vlen()) == vlen)
            class=("hover:bg-gray-100", move || !is_started())
            class=("hover:cursor-pointer", move || !is_started())
            prop:disabled=is_started
            on:click=move |_| {
                selected_vlen.set(*vlen);
            }
        >

            {vlen.to_string()}
        </div>
    }
}

use eeric::prelude::*;
use leptos::*;

use crate::widgets::registers_view::vector_view::SEWType;

use super::{FrontEndVLEN, FrontEndSEW, FrontEndLMUL};

#[component]
pub fn VectorConfig(
    cx: Scope, 
    selected_vlen: RwSignal<FrontEndVLEN>,
    engine_sew: FrontEndSEW,
    engine_lmul: FrontEndLMUL,
) -> impl IntoView {
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
                    <VlenSelector vlen={FrontEndVLEN(VLEN::V64)} current_vlen={selected_vlen}/>
                    <VlenSelector vlen={FrontEndVLEN(VLEN::V128)} current_vlen={selected_vlen}/>
                    <VlenSelector vlen={FrontEndVLEN(VLEN::V256)} current_vlen={selected_vlen}/>
                    // Impossible to style :/
                    // <VlenSelector vlen={FrontEndVLEN(VLEN::V512)} current_vlen={selected_vlen}/>
                </div>
            </div>
            <div style="grid-area: sew" class="flex flex-col justify-center items-center font-bold">
                Machine SEW = {move || engine_sew.to_string()}
            </div>
            <div style="grid-area: lmul" class="flex justify-center items-center font-bold">
                Machine LMUL = {move || engine_lmul.to_string()} 
            </div>
        </div>
    }
}

#[component]
pub fn VlenSelector(cx: Scope, vlen: FrontEndVLEN, current_vlen: RwSignal<FrontEndVLEN>) -> impl IntoView {
    view! {
        cx,
            <div
                class="px-4 py-2 hover:bg-gray-100 select-none"
                class=("font-bold", move || current_vlen() == vlen)
                class=("bg-gray-100", move || current_vlen() == vlen)
                on:click=move |_| {
                    current_vlen.set(vlen);
                }
            >
                {vlen.to_string()}
            </div>
        }
}


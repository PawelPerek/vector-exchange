use eeric::prelude::*;
use leptos::*;

use crate::widgets::MachineState;

#[component]
pub fn MemoryView(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<MachineState>>(cx);
    let memory = create_read_slice(cx, core, |state| {
        state
            .read_core()
            .map(|machine| machine.memory.snapshot())
            .unwrap_or_default()
    });

    view! { cx, 
        <div class="bg-white rounded p-4 shadow-xl max-h-[75%] overflow-y-scroll">
            <h1 class="font-bold text-center border border-gray-200 py-6">Memory</h1>
            <div class="grid grid-cols-[repeat(48,minmax(0,max-content))] px divide-x divide-y border border-gray-200">
                {move || {
                    memory()
                        .iter()
                        .enumerate()
                        .map(|(address, byte)| { 
                            view! {
                                cx, 
                                <>
                                <div class="px-0.5">
                                    {byte.to_string()}
                                </div>
                                <>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </div> }
}

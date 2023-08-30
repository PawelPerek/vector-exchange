mod integer_registers;
mod float_registers;
mod scalar_register;

use eeric::prelude::*;
use leptos::*;

use integer_registers::IntegerRegisters;
use float_registers::FloatRegisters;

#[component]
pub fn RegistersView(
    cx: Scope,
    registers_snapshot: ReadSignal<Option<RegistersSnapshot>>,
) -> impl IntoView {
    view! {
        cx,
        <div
            style="grid-area: reg"
            class="flex flex-col justify-around items-center bg-gray-200"
        >
            Registers View
            {move ||
                match registers_snapshot() {
                    None => view! { cx, <div>Registers not loaded</div> },
                    Some(snapshot) => view! {cx,
                        <div>
                            <IntegerRegisters x_regs=snapshot.x/>
                            <FloatRegisters f_regs=snapshot.f/>
                        </div>
                    }
                }
            }
        </div>
    }
}
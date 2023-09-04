mod scalar_register;
mod integer_registers;
mod float_registers;

use eeric::prelude::*;
use leptos::*;

use scalar_register::ScalarRegister;
use integer_registers::IntegerRegisters;
use float_registers::FloatRegisters;

#[component]
pub fn ScalarView(
    cx: Scope,
    snapshot: RegistersSnapshot
) -> impl IntoView {
    let pc = snapshot.pc;
    let x = snapshot.x;
    let f = snapshot.f;

    view! {cx,
        <>
            <div class="bg-white rounded p-4 shadow-xl">
                <ScalarRegister value=pc.to_string() name="pc".to_owned()/>
            </div>
            <IntegerRegisters x_regs=x />
            <FloatRegisters f_regs=f />
        </>
    }
}
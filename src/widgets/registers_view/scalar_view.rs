mod scalar_register;
mod pc_register;
mod integer_registers;
mod float_registers;

use leptos::*;

use pc_register::PcRegister;
use integer_registers::IntegerRegisters;
use float_registers::FloatRegisters;

#[component]
pub fn ScalarView(
    cx: Scope
) -> impl IntoView {
    view! {cx,
        <>
            <PcRegister />
            <IntegerRegisters />
            <FloatRegisters />
        </>
    }
}
mod scalar_register;
mod pc_register;
mod integer_registers;
mod float_registers;

use eeric::prelude::*;
use leptos::*;

use pc_register::PcRegister;
use integer_registers::IntegerRegisters;
use float_registers::FloatRegisters;

#[component]
pub fn ScalarView(
    cx: Scope
) -> impl IntoView {
    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);
    let regs = create_read_slice(
        cx, 
        core, 
        |state| state.as_ref().map(|machine| machine.registers.snapshot()).unwrap_or_default()
    );
    
    view! {cx,
        <>
            <PcRegister />
            <IntegerRegisters />
            <FloatRegisters />
        </>
    }
}
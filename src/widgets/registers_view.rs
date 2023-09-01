mod registers_header;
mod integer_registers;
mod float_registers;
mod scalar_register;

use eeric::prelude::*;
use leptos::*;

use registers_header::RegistersHeader;
use integer_registers::IntegerRegisters;
use float_registers::FloatRegisters;

#[derive(PartialEq, Clone, Copy)]
pub enum RegisterRoute {
    ScalarRegisters,
    VectorRegisters,
    CsrRegisters,
    Memory
}

impl ToString for RegisterRoute {
    fn to_string(&self) -> String {
        match self {
            Self::ScalarRegisters => "Scalar",
            Self::VectorRegisters => "Vector",
            Self::CsrRegisters => "CSR",
            Self::Memory => "Memory"
        }.to_owned()
    }
}

#[component]
pub fn RegistersView(
    cx: Scope,
    registers_snapshot: ReadSignal<Option<RegistersSnapshot>>,
) -> impl IntoView {
    let active_route = create_rw_signal(cx, RegisterRoute::ScalarRegisters);

    view! {
        cx,
        <div
            style="grid-area: reg"
            class="flex flex-col items-center bg-gray-200"
        >
            <RegistersHeader active_route={active_route}/>
            {move ||
                match registers_snapshot() {
                    None => view! { cx, <div>Registers not loaded</div> },
                    Some(snapshot) => view! {cx,
                        <div class="h-full flex flex-col justify-evenly">
                            <IntegerRegisters x_regs=snapshot.x/>
                            <FloatRegisters f_regs=snapshot.f/>
                        </div>
                    }
                }
            }
        </div>
    }
}
mod float_registers;
mod integer_registers;
mod registers_header;
mod scalar_register;

use eeric::prelude::*;
use leptos::*;

use scalar_register::ScalarRegister;
use float_registers::FloatRegisters;
use integer_registers::IntegerRegisters;
use registers_header::RegistersHeader;

#[derive(PartialEq, Clone, Copy)]
pub enum RegisterRoute {
    ScalarRegisters,
    VectorRegisters,
    CsrRegisters,
    Memory,
}

impl ToString for RegisterRoute {
    fn to_string(&self) -> String {
        match self {
            Self::ScalarRegisters => "Scalar",
            Self::VectorRegisters => "Vector",
            Self::CsrRegisters => "CSR",
            Self::Memory => "Memory",
        }
        .to_owned()
    }
}


#[component]
pub fn RegistersView<Snapshot>(
    cx: Scope,
    snapshot: Snapshot
) -> impl IntoView
    where 
        Snapshot: Fn() -> RegistersSnapshot + 'static
{
    use RegisterRoute as Route;

    let active_route = create_rw_signal(cx, Route::ScalarRegisters);

    view! {
        cx,
        <div
            style="grid-area: reg"
            class="flex flex-col items-center bg-gray-200"
        >
            <RegistersHeader active_route={active_route}/>
            <div class="grow flex flex-col justify-evenly items-center">
                {move || match active_route() {
                        Route::ScalarRegisters => {
                            view! {cx,
                                    <>
                                        <ScalarRegister value=snapshot().pc.to_string() name="pc".to_owned()/>
                                        <IntegerRegisters x_regs=snapshot().x/>
                                        <FloatRegisters f_regs=snapshot().f/>
                                    </>
                                }
                            },
                        Route::VectorRegisters => view! {cx, <>Vector todo</>},
                        Route::CsrRegisters => view! {cx, <>Csr todo</>},
                        Route::Memory => view! {cx, <>Memory todo</>},
                    }
                }
            </div>
        </div>
    }
}

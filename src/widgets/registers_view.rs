mod registers_header;
mod scalar_view;
mod vector_view;
mod csr_view;
mod memory_view;

use eeric::prelude::*;
use leptos::*;

use registers_header::RegistersHeader;
use scalar_view::ScalarView;
use vector_view::VectorView;
use csr_view::CsrView;
use memory_view::MemoryView;

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
pub fn RegistersView<Reg, Vu, Mem>(
    cx: Scope,
    reg_snapshot: Reg,
    vu_snapshot: Vu,
    mem_snapshot: Mem,
) -> impl IntoView 
    where
        Reg: Fn() -> RegistersSnapshot + 'static,
        Vu: Fn() -> VectorEngine + 'static,
        Mem: Fn() -> Vec<u8> + 'static
{
    use RegisterRoute as Route;
    let active_route = create_rw_signal(cx, Route::VectorRegisters);

    view! {
        cx,
        <div
            style="grid-area: reg"
            class="flex flex-col items-center bg-gray-200"
        >
            <RegistersHeader active_route={active_route}/>
            <div class="grow w-full flex flex-col justify-evenly items-center">
                {move || match active_route() {
                        Route::ScalarRegisters => view! {cx, <ScalarView snapshot=reg_snapshot().clone() /> },
                        Route::VectorRegisters => view! {cx, <VectorView 
                            vu_snapshot=vu_snapshot().clone()
                            reg_snapshot=reg_snapshot().clone()
                        /> },
                        Route::CsrRegisters => view! {cx, <CsrView 
                            snapshot=reg_snapshot().clone()
                        /> },
                        Route::Memory => view! {cx, <MemoryView 
                            snapshot=mem_snapshot().clone()
                        />},
                    }
                }
            </div>
        </div>
    }
}

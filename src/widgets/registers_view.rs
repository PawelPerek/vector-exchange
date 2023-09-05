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
pub fn RegistersView(
    cx: Scope,
) -> impl IntoView {
    use RegisterRoute as Route;
    let active_route = create_rw_signal(cx, Route::ScalarRegisters);

    view! {
        cx,
        <div
            style="grid-area: reg"
            class="flex flex-col items-center bg-gray-200"
        >
            <RegistersHeader active_route={active_route}/>
            <div class="grow w-full flex flex-col justify-evenly items-center">
                {move || match active_route() {
                        Route::ScalarRegisters => view! {cx, <ScalarView /> },
                        Route::VectorRegisters => view! {cx, <VectorView /> },
                        Route::CsrRegisters => view! {cx, <CsrView /> },
                        Route::Memory => view! {cx, <MemoryView />},
                    }
                }
            </div>
        </div>
    }
}

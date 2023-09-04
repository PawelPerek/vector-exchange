use eeric::prelude::*;
use leptos::*;

use super::{FrontEndVLEN, FrontEndSEW, FrontEndLMUL, SEWType};


#[component]
pub fn VectorRegisters(
    cx: Scope,
    vlen: ReadSignal<FrontEndVLEN>,
    engine_sew: SEW,
    engine_lmul: LMUL,
    v_regs: Vec<u8>
) -> impl IntoView 
{
    let (sew, set_sew) = create_signal(cx, FrontEndSEW::Default);
    let (lmul, set_lmul) = create_signal(cx, FrontEndLMUL::Default);

    view! {
        cx,
        <div class="bg-white rounded p-4 shadow-xl">
            <h1 class="font-bold text-center border border-gray-200 p-6">Vector registers</h1>
            <div class="grid grid-cols-2 border border-gray-200">
            {move || if v_regs.is_empty() {
                std::iter::repeat(0).take(32).enumerate().map(|(index, _)| {
                    view! {
                        cx, 
                        <>
                            <SingleRegister
                                index=index
                                vreg=vec![0; vlen().byte_length()] 
                                sew=sew
                                engine_sew=engine_sew
                                lmul=lmul
                                engine_lmul=engine_lmul
                            />
                        </>
                    }
                }).collect::<Vec<_>>()
            } else {
                v_regs.chunks(vlen().byte_length()).enumerate().map(|(index, vreg)| {
                    view! {
                        cx, 
                        <>
                            <SingleRegister
                                index={index}
                                vreg={vreg.iter().cloned().collect::<Vec<_>>()} 
                                sew=sew
                                engine_sew=engine_sew
                                lmul=lmul
                                engine_lmul=engine_lmul
                            />
                        </>
                    }
                }).collect::<Vec<_>>()
            }}
            </div>
        </div>
    }
}

#[component]
fn SingleRegister(
    cx: Scope,
    index: usize,
    vreg: Vec<u8>, 
    sew: ReadSignal<FrontEndSEW>,
    engine_sew: SEW,
    lmul: ReadSignal<FrontEndLMUL>,
    engine_lmul: LMUL
) -> impl IntoView {
    view! {
        cx,
        <div class="flex h-8 divide-y">
            <div class="w-12 grid justify-center items-center bg-gray-200">{vreg_name(index)}</div>
            <div class="flex divide-x">
                {move || vreg_view(
                    &vreg, 
                    sew(), 
                    engine_sew,
                    lmul(), 
                    engine_lmul
                ).into_iter().map(|vreg_value| {
                    view! {
                        cx,
                        <div class="grid justify-center items-center bg-white px-1">{vreg_value}</div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }   
}

fn vreg_view(
    bytes: &Vec<u8>, 
    sew: FrontEndSEW, 
    engine_sew: SEW,
    lmul: FrontEndLMUL,
    engine_lmul: LMUL
) -> Vec<String> {
    match sew.map_default(engine_sew) {
        (SEW::E8, SEWType::Int) => bytes
            .iter()
            .cloned()
            .array_chunks()
            .map(u8::from_le_bytes)
            .map(|byte| ToString::to_string(&byte))
            .collect::<Vec<_>>(),
        (SEW::E16, SEWType::Int) => bytes
            .iter()
            .cloned()
            .array_chunks()
            .map(u16::from_le_bytes)
            .map(|byte| ToString::to_string(&byte))
            .collect::<Vec<_>>(),
        (SEW::E32, SEWType::Int) => bytes
            .iter()
            .cloned()
            .array_chunks()
            .map(u32::from_le_bytes)
            .map(|byte| ToString::to_string(&byte))
            .collect::<Vec<_>>(),
        (SEW::E64, SEWType::Int) => bytes
            .iter()
            .cloned()
            .array_chunks()
            .map(u64::from_le_bytes)
            .map(|byte| ToString::to_string(&byte))
            .collect::<Vec<_>>(),
        (SEW::E32, SEWType::Fp) => bytes
            .iter()
            .cloned()
            .array_chunks()
            .map(u32::from_le_bytes)
            .map(|byte| ToString::to_string(&byte))
            .collect::<Vec<_>>(),
        (SEW::E64, SEWType::Fp) => bytes
            .iter()
            .cloned()
            .array_chunks()
            .map(u64::from_le_bytes)
            .map(|byte| ToString::to_string(&byte))
            .collect::<Vec<_>>(),
        _ => unreachable!()
    }
}

fn vreg_name(index: usize) -> String {
    match index {
        0 => "v0",
        1 => "v1",
        2 => "v2",
        3 => "v3",
        4 => "v4",
        5 => "v5",
        6 => "v6",
        7 => "v7",
        8 => "v8",
        9 => "v9",
        10 => "v10",
        11 => "v11",
        12 => "v12",
        13 => "v13",
        14 => "v14",
        15 => "v15",
        16 => "v16",
        17 => "v17",
        18 => "v18",
        19 => "v19",
        20 => "v20",
        21 => "v21",
        22 => "v22",
        23 => "v23",
        24 => "v24",
        25 => "v25",
        26 => "v26",
        27 => "v27",
        28 => "v28",
        29 => "v29",
        30 => "v30",
        31 => "v31",
        _ => "?",
    }
    .to_owned()
}
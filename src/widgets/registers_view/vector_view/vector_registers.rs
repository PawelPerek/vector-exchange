use eeric::prelude::*;
use leptos::*;

use super::{FrontEndSEW, SEWType};

#[component]
pub fn VectorRegisters(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);
    let vregs = create_read_slice(cx, core, |state| {
        state
            .as_ref()
            .map(|machine| machine.registers.snapshot().v)
            .unwrap_or_default()
    });

    let vec_engine = create_read_slice(cx, core, |state| {
        state
            .as_ref()
            .map(|machine| machine.vec_engine.snapshot())
            .unwrap_or_default()
    });

    let selected_vlen = expect_context::<RwSignal<VLEN>>(cx);

    let vlen_view = move || if vregs().is_empty() { selected_vlen() } else { vec_engine().vlen };

    let (sew, _set_sew) = create_signal(cx, FrontEndSEW::Default);

    let grid_cols = move || 
        (match vlen_view() {
            VLEN::V512 => 1,
            VLEN::V256 => 1,
            VLEN::V128 => 2,
            VLEN::V64 => 2,
        }) * 
        (vlen_view().byte_length() / sew().map_default(vec_engine().sew).0.byte_length() + 1);

    create_effect(cx, move |_| {
        log!("{}", grid_cols());
    });

    view! {
        cx,
        <div class="bg-white rounded p-4 shadow-xl max-h-[75%] overflow-y-scroll">
            <h1 class="font-bold text-center border border-gray-200 p-6">Vector registers</h1>
            <div
                class="grid border border-gray-200 divide-x divide-y"
                style=move || format!("grid-template-columns: repeat({}, max-content)", grid_cols())
                >
            {move || if vregs().is_empty() {
                std::iter::repeat(0).take(32).enumerate().map(|(index, _)| {
                    view! {
                        cx,
                        <>
                            <SingleRegister
                                index=index
                                vreg=vec![0; selected_vlen().byte_length()]
                                vlen=vlen_view()
                                sew=sew().map_default(vec_engine().sew)
                            />
                        </>
                    }
                }).collect::<Vec<_>>()
            } else {
                vregs().chunks(vec_engine().vlen.byte_length()).enumerate().map(|(index, vreg)| {
                    view! {
                        cx,
                        <>
                            <SingleRegister
                                index=index
                                vreg={vreg.to_vec()}
                                vlen=vlen_view()
                                sew=sew().map_default(vec_engine().sew)
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
    vlen: VLEN,
    sew: (SEW, SEWType)
) -> impl IntoView {
    let has_large_content = vlen == VLEN::V512 && sew.0 == SEW::E8;

    view! {
        cx,
        <>
            <div
                class="text-center py-1 bg-gray-200"
                class=("w-12", move || !has_large_content)
                class=("text-xs", move || has_large_content)
                class=("w-8", move || has_large_content)
            >
                {vreg_name(index)}
            </div>
            {move || vreg_view(
                &vreg,
                sew
            ).into_iter().map(|vreg_value| {
                view! {
                    cx,
                    <div
                        class="text-center p-1"
                        class=("text-xs", move || has_large_content)
                    >{vreg_value}</div>
                }
            }).collect::<Vec<_>>()}
        </>
    }
}

fn vreg_view(bytes: &[u8], sew: (SEW, SEWType)) -> Vec<String> {
    match sew {
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
        _ => unreachable!(),
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

use leptos::*;

use super::{FrontEndVLEN, FrontEndSEW, FrontEndLMUL};


#[component]
pub fn VectorRegisters(
    cx: Scope,
    vlen: ReadSignal<FrontEndVLEN>, 
    v_regs: Vec<u8>
) -> impl IntoView 
{
    let (sew, set_sew) = create_signal(cx, FrontEndSEW::Default);
    let (lmul, set_lmul) = create_signal(cx, FrontEndLMUL::Default);

    view! {
    cx,
    <div>
        <h1 class="font-bold m-6">Vector registers</h1>
        {move || if v_regs.is_empty() {
            std::iter::repeat(0).take(32).map(|_| {
                view! {
                    cx, 
                    <>
                        <SingleRegister vreg=vec![0; vlen().byte_length()] sew=sew lmul=lmul/>
                    </>
                }
            }).collect::<Vec<_>>()
        } else {
            v_regs.chunks(vlen().byte_length()).map(|vreg| {
                view! {
                    cx, 
                    <>
                        <SingleRegister vreg={vreg.iter().cloned().collect::<Vec<_>>()} sew=sew lmul=lmul/>
                    </>
                }
            }).collect::<Vec<_>>()
        }}
    </div>
    }
}

#[component]
fn SingleRegister(
    cx: Scope, 
    vreg: Vec<u8>, 
    sew: ReadSignal<FrontEndSEW>, 
    lmul: ReadSignal<FrontEndLMUL>
) -> impl IntoView {
    
} 
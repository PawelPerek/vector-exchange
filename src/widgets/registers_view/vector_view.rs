mod vector_config;
mod vector_registers;

use std::ops::Deref;

use eeric::prelude::*;
use leptos::*;

use vector_config::VectorConfig;
use vector_registers::VectorRegisters;


#[component]
pub fn VectorView(
    cx: Scope, 
    vu_snapshot: VectorEngine,
    reg_snapshot: RegistersSnapshot,
) -> impl IntoView {
    let selected_vlen = create_rw_signal(cx, FrontEndVLEN(VLEN::V128));


    view! {cx,
        <>
            <VectorConfig 
                selected_vlen=selected_vlen
                engine_sew=FrontEndSEW::Int(vu_snapshot.sew)
                engine_lmul=FrontEndLMUL::Exact(vu_snapshot.lmul)    
            />
            <VectorRegisters vlen=selected_vlen.read_only() v_regs=reg_snapshot.v />
        </>
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct FrontEndVLEN(VLEN);

impl ToString for FrontEndVLEN {
    fn to_string(&self) -> String {
        match self.0 {
            VLEN::V64 => "64b",
            VLEN::V128 => "128b",
            VLEN::V256 => "256b",
            VLEN::V512 => "512b",
        }.to_string()
    }
}

impl Deref for FrontEndVLEN {
    type Target = VLEN;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub enum FrontEndSEW {
    Default,
    Int(SEW),
    Fp(SEW)
}

impl ToString for FrontEndSEW {
    fn to_string(&self) -> String {
        match self {
            Self::Default => "Same as vector engine",
            Self::Int(SEW::E8) => "8b",
            Self::Int(SEW::E16) => "16b",
            Self::Int(SEW::E32) => "32b",
            Self::Int(SEW::E64) => "64b",
            Self::Int(SEW::E128) => "128b",
            Self::Fp(SEW::E8) => "8b (fp)",
            Self::Fp(SEW::E16) => "16b (fp)",
            Self::Fp(SEW::E32) => "32b (fp)",
            Self::Fp(SEW::E64) => "64b (fp)",
            Self::Fp(SEW::E128) => "128b (fp)",
        }.to_owned()
    }
}

pub enum FrontEndLMUL {
    Default,
    Exact(LMUL)
}

impl ToString for FrontEndLMUL {
    fn to_string(&self) -> String {
        match self {
            Self::Default => "Same as vector engine",
            Self::Exact(LMUL::MF8) => "1/8",
            Self::Exact(LMUL::MF4) => "1/4",
            Self::Exact(LMUL::MF2) => "1/2",
            Self::Exact(LMUL::M1) =>  "1",
            Self::Exact(LMUL::M2) =>  "2",
            Self::Exact(LMUL::M4) =>  "4",
            Self::Exact(LMUL::M8) =>  "8",
            Self::Exact(LMUL::M16) =>  "16",
        }.to_owned()
    }
}

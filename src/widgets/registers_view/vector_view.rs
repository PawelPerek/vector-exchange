mod vector_config;
mod vector_registers;

use std::ops::Deref;

use eeric::prelude::*;
use leptos::*;

use vector_config::VectorConfig;
use vector_registers::VectorRegisters;

#[component]
pub fn VectorView(cx: Scope) -> impl IntoView {
    view! {cx,
        <>
            <VectorConfig />
            <VectorRegisters />
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
        }
        .to_string()
    }
}

impl Deref for FrontEndVLEN {
    type Target = VLEN;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FrontEndSEW {
    Default,
    Exact((SEW, SEWType)),
}

#[derive(Clone, Copy, PartialEq)]
pub enum SEWType {
    Int,
    Fp,
}

impl FrontEndSEW {
    fn map_default(&self, default: SEW) -> (SEW, SEWType) {
        match self {
            Self::Default => (default, SEWType::Int),
            Self::Exact(sew_pair) => *sew_pair,
        }
    }
}

impl ToString for FrontEndSEW {
    fn to_string(&self) -> String {
        match self {
            Self::Default => "Same as vector engine",
            Self::Exact((SEW::E8, SEWType::Int)) => "8b",
            Self::Exact((SEW::E16, SEWType::Int)) => "16b",
            Self::Exact((SEW::E32, SEWType::Int)) => "32b",
            Self::Exact((SEW::E64, SEWType::Int)) => "64b",
            Self::Exact((SEW::E128, SEWType::Int)) => "128b",
            Self::Exact((SEW::E8, SEWType::Fp)) => "8b (fp)",
            Self::Exact((SEW::E16, SEWType::Fp)) => "16b (fp)",
            Self::Exact((SEW::E32, SEWType::Fp)) => "32b (fp)",
            Self::Exact((SEW::E64, SEWType::Fp)) => "64b (fp)",
            Self::Exact((SEW::E128, SEWType::Fp)) => "128b (fp)",
        }
        .to_owned()
    }
}

#[derive(Clone, Copy)]
pub enum FrontEndLMUL {
    Default,
    Exact(LMUL),
}

impl FrontEndLMUL {
    fn map_default(&self, default: LMUL) -> LMUL {
        match self {
            Self::Default => default,
            Self::Exact(lmul) => *lmul,
        }
    }
}

impl ToString for FrontEndLMUL {
    fn to_string(&self) -> String {
        match self {
            Self::Default => "Same as vector engine",
            Self::Exact(LMUL::MF8) => "1/8",
            Self::Exact(LMUL::MF4) => "1/4",
            Self::Exact(LMUL::MF2) => "1/2",
            Self::Exact(LMUL::M1) => "1",
            Self::Exact(LMUL::M2) => "2",
            Self::Exact(LMUL::M4) => "4",
            Self::Exact(LMUL::M8) => "8",
            Self::Exact(LMUL::M16) => "16",
        }
        .to_owned()
    }
}

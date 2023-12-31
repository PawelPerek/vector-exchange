mod editor;
mod examples_repo;
mod top_bar;

use std::collections::HashMap;

use editor::Editor;
use top_bar::TopBar;
use examples_repo::get_example;
use eeric::prelude::*;
use eeric_interpreter::prelude::*;
use leptos::{leptos_dom::log, *};

use crate::widgets::MachineState;

use super::Highlight;

#[component]
pub fn ProgramView(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<MachineState>>(cx);
    let core_exists = create_read_slice(cx, core, |machine| machine.is_on());

    let example = create_rw_signal(cx, Example::Memcpy);

    let code = create_rw_signal(cx, "".to_owned());
    let (errors, set_errors) = create_signal(cx, HashMap::<usize, String>::new());
    let (instruction_map, set_instruction_map) = create_signal(cx, vec![]);

    create_effect(cx, move |_| {
        let example = get_example(example()).to_owned();
        code.set(example);
    });

    create_effect(cx, move |_| {
        log!("{:?}", errors());
    });

    create_effect(cx, move |_| {
        // Without this log of Signal<Code> it doesn't refresh in editor, hmm...
        log!("{:?}", code());
    });

    view! { cx,
        <div
            style="grid-area: pro"
            class="flex flex-col justify-center items-center content-center"
        >
            <TopBar example={example}/>
            <Editor code=code/>
            <div class="flex w-full p-4 justify-between bg-zinc-800">
                <ResetButton/>

                {move || {
                    if core_exists() {
                        view! { cx, <StepButton instruction_map=instruction_map/> }
                    } else {
                        view! { cx,
                            <StartButton
                                code=code.read_only()
                                set_errors=set_errors
                                set_instruction_map=set_instruction_map
                            />
                        }
                    }
                }}

            </div>
        </div>
    }
}

#[component]
fn ResetButton(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<MachineState>>(cx);
    let highlighted_line = expect_context::<RwSignal<Highlight>>(cx);
    let (is_started, reset) = create_slice(
        cx,
        core,
        |state| state.is_on(),
        move |state, _: ()| {
            *state = MachineState::Off;
            highlighted_line.set(Highlight::Off);
        }
    );

    view! { cx,
        <button
            prop:disabled=move || !is_started()
            class="rounded-md px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm"
            class=("bg-zinc-400", move || !is_started())
            class=("text-zinc-600", move || !is_started())
            class=("bg-red-700", is_started)
            class=("hover:bg-red-600", is_started)
            on:click=move |_| reset(())
        >
            Reset
        </button>
    }
}

#[component]
fn StartButton(
    cx: Scope,
    code: ReadSignal<String>,
    set_errors: WriteSignal<HashMap<usize, String>>,
    set_instruction_map: WriteSignal<Vec<usize>>
) -> impl IntoView {
    let core = expect_context::<RwSignal<MachineState>>(cx);
    let vlen = expect_context::<RwSignal<Vlen>>(cx);
    let highlighted_line = expect_context::<RwSignal<Highlight>>(cx);
    let build_machine = create_write_slice(cx, core, move |machine, (instructions, memory)| {
        let vu = VectorEngineBuilder::default().vlen(vlen()).build();
        
        let core = RvCoreBuilder::default()
            .vec_engine(vu)
            .instructions(instructions)
            .memory(memory)
            .build();

        *machine = MachineState::On(core);
    });

    view! { cx,
        <button
            class="rounded-md bg-white/10 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-white/20"
            on:click=move |_| {
                let compile_result = Interpreter::compile(code(), 0x100);
                match compile_result {
                    Err(vec) => set_errors(vec),
                    Ok(result) => {
                        build_machine((result.instructions, result.memory));
                        let map = result.instructions_addresses;
                        if let Some(first) = map.first() {
                            highlighted_line.set(Highlight::On(*first + 1));
                        }
                        set_instruction_map(map);
                    }
                }
            }
        >

            Compile
        </button>
    }
}

#[component]
fn StepButton(
    cx: Scope,
    instruction_map: ReadSignal<Vec<usize>>
) -> impl IntoView {
    let core = expect_context::<RwSignal<MachineState>>(cx);
    let highlighted_line = expect_context::<RwSignal<Highlight>>(cx);
    let (_runtime_error, set_runtime_error) = create_signal(cx, None::<String>);
    
    let machine_step = create_write_slice(cx, core, move |machine, _: ()| {
        let option = machine
            .rw_core()
            .unwrap()
            .step();

        let mut did_finish = false;

        match (machine.read_core(), option) {
            // Machine executed step properly
            (Some(machine), Some(Ok(()))) => {
                let instruction_line = machine.registers.pc / 4;
                let instruction_index = instruction_map().get(instruction_line as usize).map(|el| el + 1);
    
                match instruction_index {
                    Some(index) => highlighted_line.set(Highlight::On(index)),
                    None => did_finish = true
                }
            },
            // Machine exists, but step returned error
            (Some(_), Some(Err(msg))) => {   
                set_runtime_error(Some(msg)); 
                *machine = MachineState::Off;
            },
            // Machine exists, but RvCore::step() returned nothing, probably impossible since
            // instruction_map().get(instruction_line) will return None first
            (Some(_), None) => did_finish = true,
            // Machine is none which should not happen in Step button, panic immediatly
            (None, _) => unreachable!()
        };

        if did_finish {
            let finished_machine = machine.clone().finish().unwrap();
            *machine = finished_machine;
            highlighted_line.set(Highlight::Off);
        }
    });

    view! { cx,
        <button
            class="rounded-md bg-green-700 px-3 text-sm font-semibold text-white shadow-sm hover:bg-green-600"
            on:click=move |_| machine_step(())
        >
            Step
        </button>
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Example {
    Memcpy,
    Strcpy,
    Strncpy,
    Strlen,
    Saxpy
}

impl ToString for Example {
    fn to_string(&self) -> String {
        match self {
            Self::Memcpy => "memcpy".to_owned(),
            Self::Strcpy => "strcpy".to_owned(),
            Self::Strncpy => "strncpy".to_owned(),
            Self::Strlen => "strlen".to_owned(),
            Self::Saxpy => "saxpy".to_owned(),
        }
    }
}
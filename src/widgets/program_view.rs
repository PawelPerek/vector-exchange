mod editor;

use std::collections::HashMap;

use editor::Editor;
use eeric::prelude::*;
use eeric_interpreter::prelude::*;
use leptos::{leptos_dom::log, *};

use super::Highlight;

#[component]
pub fn ProgramView(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);
    let core_exists = create_read_slice(cx, core, |machine| machine.is_some());

    let (code, set_code) = create_signal(cx, "".to_owned());
    let (errors, set_errors) = create_signal(cx, HashMap::<usize, String>::new());
    let (instruction_map, set_instruction_map) = create_signal(cx, vec![]);

    create_effect(cx, move |_| {
        log!("{:?}", errors());
    });

    view! {
        cx,
        <div
            style="grid-area: pro"
            class="flex flex-col justify-center items-center content-center"
        >
            <Editor set_code=set_code/>
            <div
                class="flex w-full p-4 justify-between bg-zinc-800"
            >
                <ResetButton />

                {move || if core_exists() {
                    view! {cx, <StepButton 
                            instruction_map=instruction_map
                        />}
                } else {
                    view! {cx, <StartButton
                            code=code
                            set_errors=set_errors
                            set_instruction_map=set_instruction_map
                        />}
                    }
                }
            </div>
        </div>
    }
}

#[component]
fn ResetButton(cx: Scope) -> impl IntoView {
    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);
    let highlighted_line = expect_context::<RwSignal<Highlight>>(cx);
    let (is_started, reset) = create_slice(
        cx,
        core,
        |state| state.is_some(),
        move |state, _: ()| {
            *state = None;
            highlighted_line.set(Highlight::Off);
        }
    );

    view! {
        cx,
        <button
            prop:disabled=move || !is_started()
            class="rounded inline-block w-fit content p-3 px-4"
            class=("bg-zinc-400", move || !is_started())
            class=("text-zinc-600", move || !is_started())
            class=("bg-red-500", is_started)
            on:click=move |_| reset(())>Reset</button>
    }
}

#[component]
fn StartButton(
    cx: Scope,
    code: ReadSignal<String>,
    set_errors: WriteSignal<HashMap<usize, String>>,
    set_instruction_map: WriteSignal<Vec<usize>>
) -> impl IntoView {
    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);
    let vlen = expect_context::<RwSignal<VLEN>>(cx);
    let highlighted_line = expect_context::<RwSignal<Highlight>>(cx);
    let build_machine = create_write_slice(cx, core, move |machine, instructions| {
        *machine = Some(
            RvCoreBuilder::default()
                .vec_engine(VectorEngineBuilder::default().vlen(vlen()).build())
                .instructions(instructions)
                .build()
        );
    });

    view! {
        cx,
        <button
            class="rounded inline-block w-fit content py-3 px-4 bg-gray-300"
            on:click=move |_| {
                let compile_result = Interpreter::compile(code());
                match compile_result {
                    Err(vec) => set_errors(vec),
                    Ok(result) => {
                        log!("{:?}", &result.instructions);

                        build_machine(result.instructions);
                        let map = result.instructions_addresses;
                        
                        if let Some(first) = map.first() {
                            highlighted_line.set(Highlight::On(*first + 1));
                        }

                        set_instruction_map(map);
                    }
                }
            }>
            Compile
        </button>
    }
}

#[component]
fn StepButton(
    cx: Scope,
    instruction_map: ReadSignal<Vec<usize>>
) -> impl IntoView {
    let core = expect_context::<RwSignal<Option<RvCore>>>(cx);
    let highlighted_line = expect_context::<RwSignal<Highlight>>(cx);
    
    let machine_step = create_write_slice(cx, core, move |machine, _: ()| {
        let result = machine
            .as_mut()
            .expect("Machine was Option::None although step button was rendered")
            .step();


        // TODO: change to match?
        if result.is_none() {
            *machine = None;
            highlighted_line.set(Highlight::Off);
        } else {
            let instruction_line = machine.as_ref().unwrap().registers.pc / 4;
            highlighted_line.set(Highlight::On(instruction_map()[instruction_line as usize] + 1));
        }
    });

    view! {
        cx,
        <button
            class="rounded inline-block w-fit content p-3 px-4 shadow-lg bg-green-500"
            on:click=move |_| machine_step(())>
            Step
        </button>
    }
}

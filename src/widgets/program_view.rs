mod editor;

use std::collections::HashMap;

use eeric::prelude::*;
use eeric_interpreter::prelude::*;
use leptos::{*, leptos_dom::log};
use editor::Editor;

#[component]
pub fn ProgramView(cx: Scope, snapshot: RwSignal<Option<RegistersSnapshot>>) -> impl IntoView {
    let (code, set_code) = create_signal(cx, "".to_owned());
    let machine = create_rw_signal(cx, None::<RvCore>);

    let (errors, set_errors) = create_signal(cx, HashMap::<usize, String>::new());

    create_effect(cx, move |_| {
        snapshot.set(machine().map(|m| m.registers_snapshot()));
    });

    create_effect(cx, move |_| {
        // TODO: attach error line and message to monaco, console.log is fair enough for now
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
                class="flex w-full p-4 justify-between border-t-gray-300 border-2"
            >
                <ResetButton machine=machine />

                {move || match machine() {
                    None => view! {cx, <StartButton 
                        code=code 
                        set_machine=machine.write_only() 
                        set_errors=set_errors
                        />},
                    Some(_) => view! {cx, <StepButton set_machine=machine.write_only() set_snapshot=snapshot.write_only() />}
                }}

                
            </div>
        </div>
    }
}

#[component]
fn ResetButton(cx: Scope, machine: RwSignal<Option<RvCore>>) -> impl IntoView {
    let is_started = move || machine().is_some();
    
    view! {
        cx,
        <button
            prop:disabled=move || !is_started()
            class="rounded border inline-block w-fit content p-3 px-4 shadow-lg"
            class=("bg-gray-400", move || !is_started())
            class=("bg-red-400", move || is_started())
            on:click=move |_| {
                machine.set(None);
            }>Reset</button>
    }
}

#[component]
fn StartButton(
    cx: Scope, 
    code: ReadSignal<String>, 
    set_machine: WriteSignal<Option<RvCore>>, 
    set_errors: WriteSignal<HashMap<usize, String>>
) -> impl IntoView {
    view! {
        cx,
        <button
            class="rounded border inline-block w-fit content py-3 px-4 shadow-lg"
            on:click=move |_| {
                let compile_result = Interpreter::compile(code());
                match compile_result {
                    Err(vec) => set_errors(vec),
                    Ok(result) => {
                        log!("Address Map: {:?}", result.instructions_addresses);
                        log!("Instructions: {:?}", result.instructions);
                        set_machine(Some(RvCore::with_instructions(result.instructions)));
                    }
                }
            }>
            Start
        </button>
    }
}

#[component]
fn StepButton(cx: Scope, set_machine: WriteSignal<Option<RvCore>>, set_snapshot: WriteSignal<Option<RegistersSnapshot>>) -> impl IntoView {
    view! {
        cx,
        <button
            class="rounded border inline-block w-fit content p-3 px-4 shadow-lg bg-green-400"
            on:click=move |_| {
                set_machine.update(|machine| {
                    let regs = machine.as_mut().unwrap().step();
                    
                    log!("{:?}", regs.as_ref().map(|r| r.x));
                    log!("{:?}", regs.as_ref().map(|r| r.pc));

                    if regs.is_none() {
                        set_machine(None);
                        set_snapshot(None);
                    }

                    set_snapshot(regs);
                })
            }>
            Step
        </button>
    }
}
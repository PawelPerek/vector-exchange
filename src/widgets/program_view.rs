use eeric::prelude::*;
use eeric_interpreter::prelude::*;
use leptos::{*, leptos_dom::log};

#[component]
pub fn ProgramView(cx: Scope, snapshot: RwSignal<Option<RegistersSnapshot>>) -> impl IntoView {
    let (code, set_code) = create_signal(cx, "".to_owned());
    let machine = create_rw_signal(cx, None::<RvCore>);

    let (errors, set_errors) = create_signal(cx, Vec::<String>::new());

    create_effect(cx, move |_| {
        snapshot.set(machine().map(|m| m.registers_snapshot()));
    });

    let is_started = move || machine.get().is_some();

    view! {
        cx,
        <div
            class="grow flex flex-col justify-center items-center content-center gap-6"
        >
            <textarea
                class="w-10/12 h-2/3 rounded-xl"
                on:input=move |ev| { 
                    set_code(event_target_value(&ev));
                }
                prop:value=code
                >
            </textarea>
            <div
                class="flex w-10/12 justify-between"
            >
                <ResetButton machine=machine />

                {move || match machine() {
                    None => view! {cx, <StartButton code=code set_machine=machine.write_only() />},
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
fn StartButton(cx: Scope, code: ReadSignal<String>, set_machine: WriteSignal<Option<RvCore>>) -> impl IntoView {
    view! {
        cx,
        <button
            class="rounded border inline-block w-fit content p-3 px-4 shadow-lg"
            on:click=move |_| {
                let instructions = Interpreter::compile(code());
                set_machine(Some(RvCore::with_instructions(instructions)));
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

                    if regs.is_none() {
                        set_machine(None);
                    }

                    set_snapshot(regs);
                })
            }>
            Step
        </button>
    }
}
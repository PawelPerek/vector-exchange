use leptos::*;

use super::Example;

#[component]
pub fn TopBar(cx: Scope, example: RwSignal<Example>) -> impl IntoView {
    let (menu_opened, set_menu_opened) = create_signal(cx, false);

    window_event_listener(ev::click, move |_| {
        set_menu_opened(false);
    });

    view! { cx,
        <div class="w-full p-4 flex items-center justify-between bg-zinc-800">
            <div class="relative inline-block text-left">
                <div>
                    <button
                        type="button"
                        class="inline-flex w-full justify-center gap-x-1.5 rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50"
                        id="menu-button"
                        aria-expanded="true"
                        aria-haspopup="true"
                        on:click=move |_| set_menu_opened.update(|menu| *menu = !*menu)
                    >
                        Examples
                        <svg
                            class="-mr-1 h-5 w-5 text-gray-400"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
                                clip-rule="evenodd"
                            ></path>
                        </svg>
                    </button>
                </div>

                <div
                    class="absolute left-0 z-10 mt-2 w-56 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                    class=("block", menu_opened)
                    class=("hidden", move || !menu_opened())
                    role="menu"
                    aria-orientation="vertical"
                    aria-labelledby="menu-button"
                    tabindex="-1"
                >
                    <div class="py-1" role="none">
                        <ExampleSelector example={Example::Memcpy} set_example={example.write_only()} />
                        <ExampleSelector example={Example::Strcpy} set_example={example.write_only()} />
                        <ExampleSelector example={Example::Strncpy} set_example={example.write_only()} />
                        <ExampleSelector example={Example::Strlen} set_example={example.write_only()} />
                        <ExampleSelector example={Example::Saxpy} set_example={example.write_only()} />
                    </div>
                </div>
            </div>

            <div class="text-white">
                Current example: {move || example().to_string()}
            </div>
        </div>
    }
}

#[component]
pub fn ExampleSelector(cx: Scope, example: Example, set_example: WriteSignal<Example>) -> impl IntoView {
    view! {
        cx,
        <a
            href="#"
            class="text-gray-700 block px-4 py-2 text-sm hover:bg-gray-100 hover:text-gray-900"
            role="menuitem"
            tabindex="-1"
            id="menu-item-0"
            on:click=move |_| {
                log!("{}", example.to_string());   
                set_example(example)
            }>
                {example.to_string()}
        </a>
    }
}
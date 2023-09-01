use leptos::*;

#[component]
pub fn ScalarRegister(cx: Scope, name: String, value: String) -> impl IntoView {
    view! {
        cx,
        <div class="border border-gray-400 w-20 m-1 divide-y rounded flex flex-col justify-center bg-white">
            <div class="text-center font-bold">{ value }</div>
            <div class="text-center">{ name }</div>
        </div>
    }
}
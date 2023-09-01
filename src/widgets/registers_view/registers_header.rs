use leptos::*;

use super::RegisterRoute;

#[component]
pub fn RegistersHeader(cx: Scope, active_route: RwSignal<RegisterRoute>) -> impl IntoView {
    view! { cx, 
        <div
            style="grid-template-columns: 1fr 3fr 1fr"
            class="w-full h-20 grid items-center bg-gray-400 divide-x"
        >
            <h1 class="text-xl font-bold text-center p-4">View</h1>
            <div class="flex justify-around">
                <Button route={RegisterRoute::ScalarRegisters} active_route={active_route}/>
                <Button route={RegisterRoute::VectorRegisters} active_route={active_route}/>
                <Button route={RegisterRoute::CsrRegisters} active_route={active_route}/>
            </div>
            <div class="flex justify-around">
                <Button route={RegisterRoute::Memory} active_route={active_route}/>
            </div>
        </div>
    }
}

#[component]
fn Button(
    cx: Scope,
    route: RegisterRoute,
    active_route: RwSignal<RegisterRoute>
) -> impl IntoView {
    view! {
        cx,
        <div 
            class="w-20 py-2 text-center bg-white rounded hover:cursor-pointer" 
            class=("font-bold", move || active_route() == route)
            on:click=move |_| {
                active_route.set(route);
            }
        >
            {route.to_string()}
        </div>
    }
}
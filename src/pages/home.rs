use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
    <>
        <h1 class="mt-5">{ "Alejandro Sempere" }</h1>
        <p class="col-6" style="text-align: justify;">{"Lorem ipsum dolor sit amet consectetur adipisicing elit. Culpa incidunt at perferendis. Atque nisi repellendus in quia itaque quo debitis asperiores ipsam a sed? Cumque odio molestias fugit eum necessitatibus!"}</p>
    </>
    }
}

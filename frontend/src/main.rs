use yew::prelude::*;
mod model;
use model::UseState;



fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Hey there"}</h2>
            <UseState />
        </div>
    }
}


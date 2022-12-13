extern crate yew;
extern crate wasm_bindgen; 

use yew::prelude::*;
use yew_router::prelude::*;
use crah::router::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}

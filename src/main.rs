use yew::prelude::*;
use yew_ethereum_provider::*;
use wasm_bindgen_futures::spawn_local;

mod components;
use components::*;

#[function_component]
pub fn App() -> Html {

    html! {
        <EthereumContextProvider>
            <ConnectButton />
        </EthereumContextProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
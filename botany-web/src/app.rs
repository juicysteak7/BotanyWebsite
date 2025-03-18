use yew::prelude::*;
use crate::Tabs;
// use reqwest::Client;
// use wasm_bindgen_futures::spawn_local;
// use wasm_bindgen::JsCast;

pub struct App {

}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Tabs/>
            </div>
        }
    }
}
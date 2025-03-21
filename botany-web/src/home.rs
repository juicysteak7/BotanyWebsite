use yew::prelude::*;

pub struct Home {

}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg:Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="home-container">
                <section class="hero">
                    <h1>{ "Welcome to Green Haven Nursery" }</h1>
                    <p>{ "Bringing nature into your home and garden." }</p>
                </section>
            </div>
        }
    }
}
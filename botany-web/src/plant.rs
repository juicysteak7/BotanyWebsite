use yew::prelude::*;


#[derive(Properties, PartialEq, Clone)]
pub struct PlantData {
    pub name: String,
    pub price: f64,
    pub image_url: String,
}

#[derive(Properties, PartialEq)]
pub struct PlantProps {
    pub name: String,
    pub price: f64,
    pub image_url: String,
}

pub struct Plant;

impl Component for Plant {
    type Message = ();
    type Properties = PlantProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html!{
            <div class="plant-card">
                <img src={props.image_url.clone()} alt={props.name.clone()} />
                <h3>{ &props.name }</h3>
                <p>{ format!("${:.2}", props.price) }</p>
                <button class="add-to-cart">{ "Add to Cart" }</button>
            </div>

        }
    }
}
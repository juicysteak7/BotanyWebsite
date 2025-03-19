use yew::prelude::*;
use std::hash::Hash;


#[derive(Properties, PartialEq, Clone)]
pub struct PlantData {
    pub plant_id: u64,
    pub name: String,
    pub price: f64,
    pub image_url: String,
}

pub enum Msg {
    AddItem(PlantData)
}

#[derive(Properties, PartialEq)]
pub struct PlantProps {
    pub plant: PlantData,
    pub add_to_cart: Callback<PlantData>,
}

pub struct Plant {
    plant: PlantData
}

impl Component for Plant {
    type Message = Msg;
    type Properties = PlantProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { plant: ctx.props().plant.clone() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddItem(item) => {
                ctx.props().add_to_cart.emit(item);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let plant = ctx.props().plant.clone();
        let plant = self.plant.clone();
        let link = ctx.link();
        html!{
            <div class="plant-card">
                <img src={plant.image_url.clone()} alt={plant.name.clone()} />
                <h3>{ &plant.name }</h3>
                <p>{ format!("${:.2}", plant.price) }</p>
                <button class="add-to-cart" onclick={link.callback(move |_| Msg::AddItem(plant.clone()))}>{ "Add to Cart" }</button>
            </div>

        }
    }
}
use yew::prelude::*;
use crate::{Plant, PlantData};

#[derive(Properties, PartialEq)]
pub struct StoreProps {
    pub plants:Vec<PlantData>,
    pub add_to_cart: Callback<PlantData>,
}

pub enum Msg {
    AddItem(PlantData),
}
pub struct Store {
    plants: Vec<PlantData>,
}

impl Component for Store {
    type Message = Msg;
    type Properties = StoreProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { plants: ctx.props().plants.clone() }
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
        let link = ctx.link();
        let _props = ctx.props();
        html!{
            <div class="store-container">
                <h2>{ "Our Plants" }</h2>
                <div class="plant-grid">
                    {
                        //iterate over plants
                        for self.plants.iter().map(|plant| {
                            html! {
                                <Plant
                                    plant={plant.clone()}
                                    add_to_cart={link.callback(|plant| Msg::AddItem(plant))}
                                />
                            }
                        })
                    }
                </div>
            </div>

        }
    }
}
use yew::prelude::*;
use crate::{Plant, PlantData};

#[derive(Properties, PartialEq)]
pub struct StoreProps {
    pub plants:Vec<PlantData>,
}

pub struct Store {
    plants: Vec<PlantData>,
}

impl Component for Store {
    type Message = ();
    type Properties = StoreProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { plants: ctx.props().plants.clone() }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
                                    name={plant.name.clone()}
                                    price={plant.price}
                                    image_url={plant.image_url.clone()}
                                />
                            }
                        })
                    }
                </div>
            </div>

        }
    }
}
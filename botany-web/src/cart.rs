use std::collections::HashMap;

use yew::prelude::*;
use crate::PlantData;

pub enum CartMsg {
    AddItem(PlantData),
}
#[derive(PartialEq, Properties)]
pub struct CartProps {
    pub list: Vec<(PlantData, u64)>,
}
pub struct Cart {
    pub list: Vec<(PlantData,u64)>,
}

impl Component for Cart {
    type Message = CartMsg;
    type Properties = CartProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { list:ctx.props().list.clone() }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // match msg {
        //     CartMsg::AddItem(item) => {
        //         self.list.push(item);
        //         true
        //     }
        // }
        true
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
            {
                for self.list.iter().map(|(plant,value)| {
                    html! {
                        <div class="cart-item">
                            <img src={plant.image_url.clone()} alt={plant.name.clone()} />
                            <h3>{ &plant.name }</h3>
                            <p>{ format!("${:.2}", plant.price) }</p>
                            <p>{ format!{"Quantity: {}", value} }</p>
                            <button class="add-to-cart">{ "Remove from Cart" }</button>
                        </div>
                    }

                })
            }
            </div>
        }
    }
}
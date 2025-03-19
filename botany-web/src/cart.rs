use yew::prelude::*;
use crate::{ PlantData, Tab };

pub enum Msg {
    RemoveItem(PlantData),
    BrowseStore,
}
#[derive(PartialEq, Properties)]
pub struct CartProps {
    pub list: Vec<(PlantData, u64)>,
    pub remove_from_cart: Callback<PlantData>,
    pub browse_button: Callback<Tab>
}
pub struct Cart {
    pub list: Vec<(PlantData,u64)>,
}

impl Component for Cart {
    type Message = Msg;
    type Properties = CartProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { list:ctx.props().list.clone() }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RemoveItem(item) => {
                let mut i = 0;
                let mut index_to_remove = None;
                for (plant, val) in self.list.iter_mut() {
                    if plant.plant_id == item.plant_id && *val > 1 {
                        *val -= 1;
                    } else if plant.plant_id == item.plant_id {
                        index_to_remove = Some(i);
                    }
                    i += 1;
                }
                if let Some(i) = index_to_remove {
                    self.list.remove(i);
                }
                ctx.props().remove_from_cart.emit(item);
                log::info!("list: {:?}", self.list.clone());
                true
            }
            Msg::BrowseStore => {
                ctx.props().browse_button.emit(Tab::Store);
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        if self.list.len() == 0 {
            return html! {
                <div class="empty-cart">
                    <img src="https://cdn-icons-png.flaticon.com/512/4555/4555974.png" alt="Empty Cart" />
                    <h3>{ "Your cart is empty." }</h3>
                    <p>{ "Browse our selection of beautiful plants and add something to your cart!" }</p>
                    <button class="browse-btn" onclick={link.callback(|_| Msg::BrowseStore)}>{ "Browse Shop" }</button>
                </div>
            }
        }
        html! {
            <div class="cart-container">
                <h2 class="cart-header">{ "Your Cart" }</h2>
                {
                    for self.list.iter().map(|(plant,value)| {
                        let plant_clone = plant.clone();
                        html! {
                            <div class="cart-item">
                                <img src={plant.image_url.clone()} alt={plant.name.clone()} />
                                <h3>{ &plant.name }</h3>
                                <p>{ format!("${:.2}", plant.price) }</p>
                                <p>{ format!{"Quantity: {}", value} }</p>
                                <button class="remove-from-cart" onclick={link.callback(move |_| Msg::RemoveItem(plant_clone.clone()))}>{ "Remove" }</button>
                            </div>
                        }

                    })
                }
            </div>
        }
    }
}
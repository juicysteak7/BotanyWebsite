use yew::prelude::*;
use crate::{ Home, Store, PlantData, Cart };

enum Tab {
    Home,
    Store,
    About,
    Contact,
    Cart,
}
pub struct Tabs {
    selected_tab: Tab,
    list: Vec<(PlantData,u64)>
}

pub enum Msg {
    ChangeTab(Tab),
    AddItem(PlantData),
    RemoveItem(PlantData),
}

impl Component for Tabs {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { selected_tab: Tab::Home, list:Vec::new() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeTab(tab) => {
                self.selected_tab = tab;
                true
            }
            Msg::AddItem(item) => {
                let mut found = false;
                for (plant, value) in self.list.iter_mut() {
                    if plant.plant_id == item.plant_id {
                        *value += 1;
                        found = true;
                    }
                }
                if !found {
                    self.list.push((item,1));
                }
                true
            }
            Msg::RemoveItem(item) => {
                let mut i = 0;
                let mut index_to_remove = None;
                for (plant, val) in self.list.iter_mut() {
                    if plant.plant_id == item.plant_id && *val > 1 {
                        *val -= 1;
                    } else if plant.plant_id == item.plant_id{
                        index_to_remove = Some(i);
                    }
                    i += 1;
                }
                if let Some(i) = index_to_remove {
                    self.list.remove(i);
                }
                log::info!("list: {:?}", self.list.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let mut plant_data:Vec<PlantData> = Vec::new();
        plant_data.push(PlantData {
            name: "Peace Lily".to_string(),
            price: 10.99,
            image_url:"https://gardendalefloral.com/wp-content/uploads/2023/04/SimplyElegantSpathiphyllum-Large.jpeg".to_string(),
            plant_id: 0,
        });
        plant_data.push(PlantData {
            name: "Aloe Vera".to_string(),
            price: 11.99,
            image_url:"data:image/jpeg;base64,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".to_string(),
            plant_id: 1,
        });
        plant_data.push(PlantData {
            name: "Snake".to_string(),
            price: 12.99,
            image_url:"data:image/jpeg;base64,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".to_string(),
            plant_id: 2,
        });

        html! {
            <div>
                <nav class="nav">
                <div class="nav-left">
                    <span class="nav-item" onclick={link.callback(|_| Msg::ChangeTab(Tab::Home))}>{ "Home" }</span>
                    <span class="nav-item" onclick={link.callback(|_| Msg::ChangeTab(Tab::Store))}>{ "Store" }</span>
                    <span class="nav-item" onclick={link.callback(|_| Msg::ChangeTab(Tab::About))}>{ "About" }</span>
                    <span class="nav-item" onclick={link.callback(|_| Msg::ChangeTab(Tab::Contact))}>{ "Contact" }</span>
                </div>
                <div class="nav-right">
                    <span class="nav-item cart" onclick={link.callback(|_| Msg::ChangeTab(Tab::Cart))}>{ "🛒" }</span>
                </div>
                </nav>
                <div>
                    {
                        match self.selected_tab {
                            Tab::Home => html! { <Home/> },
                            Tab::About => html! { <p>{ "About this project..." }</p> },
                            Tab::Contact => html! { <p>{ "Contact us at..." }</p> },
                            Tab::Store => html! { <Store plants={plant_data}
                                                    add_to_cart={link.callback(|plant| Msg::AddItem(plant))}/> },
                            Tab::Cart => html! { <Cart list={self.list.clone()}
                                                    remove_from_cart={link.callback(|plant| Msg::RemoveItem(plant))}/> }
                        }
                    }
                </div>
            </div>
        }
    }
}
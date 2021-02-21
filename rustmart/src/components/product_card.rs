use crate::types::Product;
use yew::prelude::*;

pub struct ProductCard {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<()>,
}

impl Component for ProductCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let product = &self.props.product;
        let onclick = self.props.on_add_to_cart.reform(|_| ());
        html! {
          <div id={product.id}>
            <img src={&product.image}/>
            <div>{"Name: "}{&product.name}</div>
            <div>{"Description: "}{&product.description}</div>
            <div>{"Price: $"}{&product.price}</div>
            <button onclick=onclick>{"Add To Cart"}</button>
          </div>
        }
    }
}

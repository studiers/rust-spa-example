use crate::components::atc_button::AtcButton;
use crate::route::Route;
use crate::types::Product;
use yew::prelude::*;
use yew_router::prelude::RouterAnchor;

pub struct ProductCard {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub product: Product,
    pub on_add_to_cart: Callback<Product>,
}

impl Component for ProductCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let product = &self.props.product;
        html! {
          <div id={product.id}>
            <RouterAnchor<Route> route=Route::ProductDetail(self.props.product.id)>
                <img src={&product.image}/>
                <div>{"Name: "}{&product.name}</div>
                <div>{"Description: "}{&product.description}</div>
                <div>{"Price: $"}{&product.price}</div>
            </RouterAnchor<Route>>
            <AtcButton product={self.props.product.clone()} on_add_to_cart=self.props.on_add_to_cart.clone() />
          </div>
        }
    }
}

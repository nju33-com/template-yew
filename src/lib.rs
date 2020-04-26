#![recursion_limit = "256"]

mod console;
mod hello;
mod ns;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Root {}

impl Component for Root {
  type Message = ();
  type Properties = ();
  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _message: Self::Message) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    let name = Rc::new("nju33".into());

    html! {
      <hello::component::Hello name=name />
    }
  }
}

#[wasm_bindgen]
pub fn render_hello() -> Result<(), JsValue> {
  yew::initialize();
  App::<Root>::new().mount_to_body();
  Ok(())
}

use crate::console;
use crate::ns;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, Clone, Default)]
pub struct Props {
  pub name: Rc<String>,
}

pub struct Hello {
  props: Props,
  link: ComponentLink<Self>,
}

pub enum Message {
  ButtonClick,
}

impl Component for Hello {
  type Message = Message;
  type Properties = Props;
  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { props, link }
  }

  fn update(&mut self, message: Self::Message) -> ShouldRender {
    match message {
      Message::ButtonClick => console::log("ButtonClick"),
    };

    true
  }

  fn change(&mut self, props: Self::Properties) -> bool {
    self.props = props;
    true
  }

  fn view(&self) -> Html {
    html! {
      <div>
        <p>{"Hello "}{self.props.name.clone()}</p>
        <p>{"The" }<code>{"baz"}</code>{" is "}{ns::get_bar()}</p>
        <button onclick=self.link.callback(|_| Message::ButtonClick)>{"button"}</button>
      </div>
    }
  }
}

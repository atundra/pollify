use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(PartialEq, Clone)]
pub struct SelectItem {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    pub items: Vec<SelectItem>,
    pub value: Option<String>,
    #[prop_or_default]
    pub class: String,
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub placeholder: Option<String>,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let onchange = {
        let props = props.clone();
        Callback::from(move |e: Event| {
            let value = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                .unwrap()
                .value();
            props.onchange.emit(value);
        })
    };

    let items = props.items.clone();
    let options: Vec<VNode> = items
        .into_iter()
        .map(|item| {
            let selected = props
                .value
                .clone()
                .map(|value| value == item.value)
                .unwrap_or_default();

            html! {
              <option value={item.value.clone()} {selected}>{item.label}</option>
            }
        })
        .collect();

    let mut placeholder_items: Vec<VNode> = vec![];
    if let Some(placeholder) = props.placeholder.clone() {
        placeholder_items.push(html! {
          <option value="_placeholder" disabled={true} selected={props.value.is_none()}>{placeholder}</option>
        });
    }

    html! {
      <select
        class={props.class.clone()}
        value={props.value.clone()}
        onchange={onchange}
      >
        {[placeholder_items, options].concat().into_iter().collect::<Html>()}
      </select>
    }
}

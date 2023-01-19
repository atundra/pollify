use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub typ: String,
    pub value: String,
    #[prop_or_default]
    pub placeholder: String,
    pub class: String,
    pub onchange: Callback<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let onchange = {
        let props = props.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .unwrap()
                .value();
            props.onchange.emit(value);
        })
    };

    html! {
      <input
        type={props.typ.clone()}
        placeholder={props.placeholder.clone()}
        class={props.class.clone()}
        value={props.value.clone()}
        oninput={onchange}
      />
    }
}

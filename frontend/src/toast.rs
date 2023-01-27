use gloo_timers::callback::Timeout;
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::{use_list, use_renders_count};

enum ToastType {
    Error,
    Info,
    Success,
}

type InnerShow = Rc<dyn Fn(&str, Option<u32>, ToastType)>;

#[derive(Clone)]
pub struct ToastHandler {
    inner_show: InnerShow,
}

#[allow(dead_code)]
impl ToastHandler {
    pub fn error(&self, message: impl AsRef<str>) {
        (self.inner_show)(message.as_ref(), None, ToastType::Error);
    }

    pub fn error_with_timeout(&self, message: impl AsRef<str>, timeout_millis: u32) {
        (self.inner_show)(message.as_ref(), Some(timeout_millis), ToastType::Error);
    }

    pub fn success(&self, message: impl AsRef<str>) {
        (self.inner_show)(message.as_ref(), None, ToastType::Success);
    }

    pub fn success_with_timeout(&self, message: impl AsRef<str>, timeout_millis: u32) {
        (self.inner_show)(message.as_ref(), Some(timeout_millis), ToastType::Success);
    }

    pub fn info(&self, message: impl AsRef<str>) {
        (self.inner_show)(message.as_ref(), None, ToastType::Info);
    }

    pub fn info_with_timeout(&self, message: impl AsRef<str>, timeout_millis: u32) {
        (self.inner_show)(message.as_ref(), Some(timeout_millis), ToastType::Info);
    }
}

impl PartialEq for ToastHandler {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[hook]
pub fn use_toast() -> Option<ToastHandler> {
    use_context::<ToastHandler>()
}

struct ActiveToast {
    id: i32,
    message: String,
    #[allow(dead_code)]
    timeout_ref: Timeout,
    toast_type: ToastType,
}

const FACTORY_DEFAULT_TIMEOUT_MILLIS: u32 = 5000;

#[derive(Properties, PartialEq)]
struct WrapperProps {
    children: Children,
}

#[derive(Properties, PartialEq)]
pub struct ToastProviderProps {
    pub children: Children,
    #[prop_or(FACTORY_DEFAULT_TIMEOUT_MILLIS)]
    pub default_timeout_millis: u32,
}

#[function_component(ToastProvider)]
pub fn toast_provider(
    ToastProviderProps {
        children,
        default_timeout_millis,
    }: &ToastProviderProps,
) -> Html {
    let active_toasts = use_list::<ActiveToast>(Default::default());
    let unque_id = use_renders_count();

    let context = {
        let active_toasts = active_toasts.clone();
        let default_timeout_millis = *default_timeout_millis;
        ToastHandler {
            inner_show: Rc::new(move |message, t, toast_type| {
                let timeout = t.unwrap_or(default_timeout_millis);
                let toast_id = unque_id;
                let active_toasts = active_toasts.clone();
                let timeout_ref = Timeout::new(timeout, {
                    let active_toasts = active_toasts.clone();
                    let toast_id = toast_id;
                    move || active_toasts.retain(|ActiveToast { id, .. }| *id != toast_id)
                });

                let new_toast = ActiveToast {
                    id: toast_id,
                    message: message.to_string(),
                    timeout_ref,
                    toast_type,
                };
                active_toasts.push(new_toast);
            }),
        }
    };

    let toasts_component: Html = active_toasts
        .current()
        .iter()
        .map(
            |ActiveToast {
                 message,
                 id,
                 toast_type,
                 ..
             }| {
                let toast_type_class = match toast_type {
                    ToastType::Error => "alert-error",
                    ToastType::Info => "alert-info",
                    ToastType::Success => "alert-success",
                };
                let class = classes!("alert", toast_type_class);
                html! {
                    <div key={*id} {class}>
                        <div>
                            <span>{message}</span>
                        </div>
                    </div>
                }
            },
        )
        .rev()
        .collect();

    html! {
      <>
        <div class="toast toast-top">{toasts_component}</div>
        <ContextProvider<ToastHandler> {context}>
          { for children.iter() }
        </ContextProvider<ToastHandler>>
      </>
    }
}

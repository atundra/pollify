use crate::{async_data::AsyncData, toast::use_toast};
use yew::prelude::*;

#[hook]
pub fn use_toast_on_async_data_error<T, E>(async_data: AsyncData<T, E>)
where
    T: PartialEq + Clone + 'static,
    E: Into<String> + PartialEq + Clone + 'static,
{
    let toast = use_toast().expect("ToastProvider is expected on component tree");

    use_effect_with_deps(
        move |data| {
            if let AsyncData::Failed(error) = data {
                toast.error(error.clone().into());
            }
        },
        async_data,
    );
}

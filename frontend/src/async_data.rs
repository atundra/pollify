use yew_hooks::UseAsyncHandle;

#[derive(PartialEq, Clone, Copy)]
pub enum AsyncData<T, E> {
    Idle,
    Loading,
    Loaded(T),
    Failed(E),
}

pub trait ToAsyncData<T, E> {
    fn to_async_data(&self) -> AsyncData<T, E>;
}

impl<T, E> ToAsyncData<T, E> for UseAsyncHandle<T, E>
where
    T: Clone,
    E: Clone,
{
    fn to_async_data(&self) -> AsyncData<T, E> {
        match (self.loading, &self.error, &self.data) {
            (false, None, None) => AsyncData::Idle,
            (true, _, _) => AsyncData::Loading,
            (false, Some(err), _) => AsyncData::Failed(err.clone()),
            (false, None, Some(result)) => AsyncData::Loaded(result.clone()),
        }
    }
}

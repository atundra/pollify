use crate::codegen::poll_service::poll_service_client::PollService;
use std::rc::Rc;
use yew::prelude::*;

static HOST: &str = "http://localhost:50051";

#[hook]
pub fn use_poll_service() -> Rc<PollService> {
    use_memo(|_| PollService::new(HOST.to_string()), ())
}

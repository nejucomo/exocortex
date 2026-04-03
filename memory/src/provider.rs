use exocortex_handler::PollHandler;

use crate::Request;

pub trait Provider: PollHandler<Request, PollError = Self::Error> {
    type Error: std::error::Error;
}

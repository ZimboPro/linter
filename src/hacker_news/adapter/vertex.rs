use std::rc::Rc;

use hn_api::types::{Comment, Item, Job, Poll, Pollopt, Story, User};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    Comment(Rc<Comment>),
    // Item(()), // Item is base
    Job(Rc<Job>),
    Poll(Rc<Poll>),
    PollOption(Rc<Pollopt>),
    Story(Rc<Story>),
    User(Rc<User>),
}

impl From<Item> for Vertex {
    fn from(item: Item) -> Self {
        match item {
            Item::Story(x) => Self::Story(x.into()),
            Item::Comment(x) => Self::Comment(x.into()),
            Item::Job(x) => Self::Job(x.into()),
            Item::Poll(x) => Self::Poll(x.into()),
            Item::Pollopt(x) => Self::PollOption(x.into()),
        }
    }
}

impl From<Comment> for Vertex {
    fn from(value: Comment) -> Self {
        Self::Comment(Rc::from(value))
    }
}

impl From<User> for Vertex {
    fn from(value: User) -> Self {
        Self::User(Rc::from(value))
    }
}

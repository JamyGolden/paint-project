use std::{cell::RefCell, collections::HashMap, fmt, thread::LocalKey};
use yew::prelude::*;

use crate::components::pattern::RawGridType;

#[derive(Clone)]
pub struct HistoryContext {
    pub history: &'static LocalKey<RefCell<Vec<RawGridType>>>,
}

impl HistoryContext {
    fn new(history: &'static LocalKey<RefCell<Vec<RawGridType>>>) -> Self {
        Self { history }
    }

    pub fn get_grid(&self) -> RawGridType {
        self.history.with(|self_history| {
            let default: RawGridType = HashMap::default();

            self_history.borrow().last().cloned().unwrap_or(default)
        })
    }
}

impl PartialEq for HistoryContext {
    fn eq(&self, other: &Self) -> bool {
        self.history.with(|self_history| {
            other
                .history
                .with(|other_history| *self_history.borrow() == *other_history.borrow())
        })
    }
}

impl fmt::Debug for HistoryContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.history.with(|history| {
            f.debug_struct("HistoryContext")
                .field("history", &history.borrow())
                .finish()
        })
    }
}

#[derive(Properties, PartialEq)]
pub struct HistoryProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(HistoryProvider)]
pub fn history_provider(props: &HistoryProviderProps) -> Html {
    thread_local! {
        static HISTORY: RefCell<Vec<RawGridType>> = RefCell::new(Vec::default());
    };
    let history = HistoryContext::new(&HISTORY);

    html! {
        <ContextProvider<HistoryContext> context={history}>
            { for props.children.iter() }
        </ContextProvider<HistoryContext>>
    }
}

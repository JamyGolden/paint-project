use html::ImplicitClone;
use implicit_clone::sync::{IArray, IMap};
use indexmap::IndexMap;
use std::{collections::HashMap, hash::Hash, rc::Rc, sync::Arc};
use yew::prelude::*;

use crate::components::pattern::{GridType, RawGridType};

pub enum PatternEditorAction {
    AddHistory(RawGridType),
    RemoveHistory,
}

#[derive(Default, PartialEq)]
pub struct PatternEditorState {
    pub history: IArray<GridType>,
    pub grid: GridType,
}

impl Reducible for PatternEditorState {
    type Action = PatternEditorAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_state: Self = match action {
            PatternEditorAction::AddHistory(grid) => {
                let mut history = self.history.to_vec();
                let grid = hashmap_to_imap(grid);

                history.push(grid.clone());

                Self {
                    history: IArray::from(history),
                    grid,
                }
            }
            PatternEditorAction::RemoveHistory => {
                let (without_last, _) = self.history.as_slice().split_at(self.history.len() - 1);

                Self {
                    history: IArray::from(without_last.to_vec()),
                    grid: self.grid.clone(),
                }
            }
        };

        next_state.into()
    }
}

fn hashmap_to_imap<K, V>(hash_map: HashMap<K, V>) -> IMap<K, V>
where
    K: Eq + Hash + ImplicitClone + 'static,
    V: PartialEq + ImplicitClone + 'static,
{
    let index_map: IndexMap<K, V> = hash_map.into_iter().collect();
    IMap::Rc(Arc::new(index_map))
}

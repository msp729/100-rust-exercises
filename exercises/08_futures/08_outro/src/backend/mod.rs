mod description;
mod status;
mod title;

pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone)]
pub struct TicketStore {
    pub tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;
    type IntoIter = std::vec::IntoIter<&'a Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        self.tickets
            .iter()
            .map(|x| x.1)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct TicketId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }

    pub fn delete(&mut self, id: TicketId) -> bool {
        self.tickets.remove(&id).is_some()
    }

    pub fn ids<'a>(&'a self) -> Vec<TicketId> {
        self.tickets.keys().copied().collect()
    }
}

impl Index<TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: TicketId) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl Index<&TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: &TicketId) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl IndexMut<&TicketId> for TicketStore {
    fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output {
        &mut self[*index]
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::{
        Status, TicketDescription, TicketDraft, TicketId, TicketStore, TicketTitle,
    };

    pub fn valid_title() -> String {
        "A title".into()
    }

    pub fn valid_description() -> String {
        "A description".into()
    }

    /// A function to generate a valid ticket title,
    /// for test purposes.
    pub fn ticket_title() -> TicketTitle {
        valid_title().try_into().unwrap()
    }

    /// A function to generate a valid ticket description,
    /// for test purposes.
    pub fn ticket_description() -> TicketDescription {
        valid_description().try_into().unwrap()
    }

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let n_tickets = 5;

        for _ in 0..n_tickets {
            let draft = TicketDraft {
                title: ticket_title(),
                description: ticket_description(),
            };
            let id = store.add_ticket(draft.clone());
            let ticket = &store[id];
            assert_eq!(draft.title, ticket.title);
            assert_eq!(draft.description, ticket.description);
            assert_eq!(ticket.status, Status::ToDo);

            let ticket = &mut store[id];
            ticket.status = Status::InProgress;

            let ticket = &store[id];
            assert_eq!(ticket.status, Status::InProgress);
        }

        let ids: Vec<TicketId> = (&store).into_iter().map(|t| t.id).collect();
        let sorted_ids = {
            let mut v = ids.clone();
            v.sort();
            v
        };
        assert_eq!(ids, sorted_ids);
    }
}

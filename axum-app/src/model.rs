use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use crate::error::{Result, Error};


//ticket
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ticket{
    pub id: u64,
    pub title: String,

}

#[derive(Debug, Clone, Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}


// Model Controller

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok( ModelController{
            tickets_store: Arc::default(),
        })
    }

}


// CRUD Implemention

impl ModelController {
    pub async fn  createticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket>{
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));
        Ok(ticket)

    }
    pub async fn  listticket(&self) -> Result<Vec<Ticket>> {
        let  store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }
    pub async fn deleteticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailedIdNotFound{id})
    }
}
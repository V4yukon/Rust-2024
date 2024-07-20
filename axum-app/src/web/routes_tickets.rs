use axum::extract::{State, Path, Json};
use crate::model::{ModelController, Ticket, TicketForCreate};
use axum::Router;
use axum::routing::{post, get, delete};
use crate::error::Result;

pub fn routes(mc: ModelController) -> Router{
    Router::new()
        .route("/ticket", post(create_ticket).get(list_ticket))
        .route("/ticket/:id", delete(delete_ticket))
        .with_state(mc)

}


// async fn create_ticket(State(mc): State<ModelController>, Json(ticket_fc): Json<TicketForCreate>) -> Result<Json<Ticket>> {
//     println!("->> {:<12} - Create-Ticket", "HANDLER");
//     let ticket = mc.createticket(ticket_fc).await?;
//     Ok(Json(ticket))

// }
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - Create-Ticket", "HANDLER");
    let ticket = mc.createticket(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_ticket(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - List-Ticket", "HANDLER");
    let ticket = mc.listticket().await?;
    Ok(Json(ticket))

}


async fn delete_ticket(State(mc): State<ModelController>, Path(id): Path<u64>) -> Result<Json<Ticket>> {
    println!("->> {:<12} - Delete-Ticket", "HANDLER");
    let ticket = mc.deleteticket(id).await?;
    Ok(Json(ticket))

}
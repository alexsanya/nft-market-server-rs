use tracing::debug;
use axum::{http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use crate::{controllers::settlement::{create_settlement, get_settlements}, dtos::settlement::{ParsingError, SettlementDTO}, error::{Entity, Error}, models::settlement::Settlement, prelude::Result};

pub fn create_route() -> Router {
    Router::new()
        .route("/settlements", post(create))
        .route("/settlements", get(get_all))
}

async fn create(Json(payload): Json<SettlementDTO>) -> Result<impl IntoResponse> {
    let result = payload.try_into();

    if let Ok(settlement) = result {
        create_settlement(&settlement).await?;
        debug!("Success");
        debug!("{:?}", settlement);
        Ok(StatusCode::CREATED)
    } else {
        let err = result.unwrap_err();
        debug!("Error {:?}", err);
        match &err {
            ParsingError::Bid(bid_err) => Err(
                Error::InvalidInput(
                    Entity::Bid,
                    format!("{}.{}", err.as_ref(), bid_err.as_ref())
                )
            ),
            ParsingError::Signature(sig_err) => Err(
                Error::InvalidInput(
                    Entity::Settlement,
                    format!("{}.{}", err.as_ref(), sig_err.as_ref())
                )
            )
        }
    }
}

async fn get_all()-> Result<Json<Vec<Settlement>>> {
    let settlements = get_settlements().await?;
    Ok(Json(settlements))
}
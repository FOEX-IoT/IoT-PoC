use crate::prelude::*;
use crate::tradfri::tradfri_messages::*;
use serde_json::json;

pub async fn get_all_lamps(ls: web::Data<Addr<TradfriServer>>) -> Result<HttpResponse, APIError> {
    let lamps = ls.send(GetAllLampsMessage {}).await??;
    Ok(HttpResponse::Ok().json(lamps))
}

pub async fn change_status_of_lamp(
    ls: web::Data<Addr<TradfriServer>>,
    bs: web::Data<Addr<BroadcastServer>>,
    update: web::Json<ChangeStatusRequest>,
) -> Result<HttpResponse, APIError> {
    let update = update.into_inner();

    let data = json!({
        "type": "LAMP",
        "values": &update,
    });

    let update_str = serde_json::to_string(&data)?;

    // send the message the tradfri server which performs the coap request
    ls.send(update).await??;

    // send the message to the websocket server which broadcasts it to all clients
    bs.send(BCMessage(update_str)).await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn change_brightness_of_lamp(
    ls: web::Data<Addr<TradfriServer>>,
    bs: web::Data<Addr<BroadcastServer>>,
    update: web::Json<ChangeBrightnessRequest>,
) -> Result<HttpResponse, APIError> {
    let update = update.into_inner();

    let data = json!({
        "type": "LAMP",
        "values": &update,
    });

    let update_str = serde_json::to_string(&data)?;

    // send the message the tradfri server which performs the coap request
    ls.send(update).await??;

    // send the message to the websocket server which broadcasts it to all clients
    bs.send(BCMessage(update_str)).await?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn change_scene(
    ls: web::Data<Addr<TradfriServer>>,
    bs: web::Data<Addr<BroadcastServer>>,
    update: web::Json<ChangeSceneRequest>,
) -> Result<HttpResponse, APIError> {
    let update = update.into_inner();

    let data = json!({
        "type": "SCENES",
        "values": &update,
    });

    let update_str = serde_json::to_string(&data)?;

    // send the message the tradfri server which performs the coap request
    ls.send(update).await??;

    // send the message to the websocket server which broadcasts it to all clients
    bs.send(BCMessage(update_str)).await?;

    Ok(HttpResponse::Ok().finish())
}

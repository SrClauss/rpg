use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::broadcast;

use crate::api::campaign::get_or_load_campaign_by_room;
use crate::api::state::ServerState;

pub fn ws_router() -> Router<ServerState> {
    Router::new().route("/ws/{room_code}", get(ws_handler))
}

pub async fn ws_handler(
    Path(room_code): Path<String>,
    ws: WebSocketUpgrade,
    State(state): State<ServerState>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_socket(socket, room_code, state))
}

pub async fn handle_socket(socket: WebSocket, room_code: String, state: ServerState) {
    let (mut sender, mut receiver) = socket.split();

    // Obtém ou cria o canal de broadcast da sala
    let rx_opt = {
        let mut channels = state.channels.write().await;
        let tx = channels
            .entry(room_code.clone())
            .or_insert_with(|| broadcast::channel(100).0);
        Some(tx.subscribe())
    };

    let mut rx = rx_opt.unwrap();

    // Envia estado inicial da sala para o cliente recém-conectado (buscando em memória ou carregando do disco sob demanda)
    if let Some(camp_arc) = get_or_load_campaign_by_room(&state, &room_code).await {
        let camp = camp_arc.read().await;
        let player_slots = camp
            .manifest
            .as_ref()
            .map(|m| m.player_slots.clone())
            .unwrap_or_default();
        let init_msg = json!({
            "type": "INITIAL_SYNC",
            "campaign_id": camp.campaign_id,
            "room_code": camp.room_code,
            "player_slots": player_slots,
            "active_scene": camp.active_scene,
        });
        let _ = sender.send(Message::Text(init_msg.to_string().into())).await;
    }

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg_text) = rx.recv().await {
            if sender.send(Message::Text(msg_text.into())).await.is_err() {
                break;
            }
        }
    });

    let room_code_clone = room_code.clone();
    let channels_clone = state.channels.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                let channels = channels_clone.read().await;
                if let Some(tx) = channels.get(&room_code_clone) {
                    let _ = tx.send(text.to_string());
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

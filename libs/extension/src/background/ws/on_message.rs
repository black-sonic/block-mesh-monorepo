use crate::background::ws::websocket::set_ws_status;
use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use crate::utils::log::{log, log_error};
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use leptos::SignalGetUntracked;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use wasm_bindgen::prelude::*;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent};

pub fn on_message_handler(
    _ws: web_sys::WebSocket,
    app_state: ExtensionWrapperState,
) -> Closure<dyn FnMut(MessageEvent)> {
    Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        log!("on_message_handle => {:#?}", e);
        let _email = app_state.email.get_untracked();
        let _api_token = app_state.api_token.get_untracked();
        // let metadata = fetch_metadata_blocking().unwrap_or_default();
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            match serde_json::from_str::<WsServerMessage>(
                &txt.as_string()
                    .unwrap_or("Couldn't covert Js String to String".to_string()),
            ) {
                Ok(msg) => {
                    log!("msg => {:#?}", msg);
                }
                Err(_error) => {}
            }
        } else {
            log_error!("message event, received Unknown: {:?}", e.data());
        }
    })
}

pub fn on_error_handler(ws: web_sys::WebSocket) -> Closure<dyn FnMut(ErrorEvent)> {
    Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        let state: WebSocketReadyState = ws.ready_state().into();
        set_ws_status(&state);
        log_error!("closing ws with error error event: {:?} | {:?}", e, state);
    })
}

pub fn on_open_handler(ws: web_sys::WebSocket) -> Closure<dyn FnMut()> {
    Closure::<dyn FnMut()>::new(move || {
        log!("socket opened");
        match ws.send_with_str("ping") {
            Ok(_) => log!("Sent a ping message."),
            Err(err) => log_error!("error sending message: {:?}", err),
        }
    })
}

pub fn on_close_handler(ws: web_sys::WebSocket) -> Closure<dyn FnMut(CloseEvent)> {
    Closure::<dyn FnMut(_)>::new(move |e: CloseEvent| {
        let state: WebSocketReadyState = ws.ready_state().into();
        set_ws_status(&state);
        log_error!("closing ws with error error event: {:?} | {:?}", e, state);
    })
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum WebSocketReadyState {
    CONNECTING,
    OPEN,
    CLOSING,
    CLOSED,
    INVALID,
}

impl From<u16> for WebSocketReadyState {
    fn from(value: u16) -> Self {
        match value {
            0 => WebSocketReadyState::CONNECTING,
            1 => WebSocketReadyState::OPEN,
            2 => WebSocketReadyState::CLOSING,
            3 => WebSocketReadyState::CLOSED,
            _ => WebSocketReadyState::INVALID,
        }
    }
}
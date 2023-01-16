use echo_core::Request;
use echo_ws::{WebSocketUpgrade, WebSocketUpgradeError};

pub fn ws(request: &mut Request) -> Result<WebSocketUpgrade, WebSocketUpgradeError> {
    WebSocketUpgrade::from_request(request)
}

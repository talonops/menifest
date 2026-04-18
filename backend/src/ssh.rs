use russh::client::{Config, Handler};
use std::sync::Arc;

use crate::routes::server::ConnectRequest;

struct Client;

impl Handler for Client {
    type Error = russh::Error;

    /*
       for production we need to have known_hosts, or list of trusted keys.
    */
    async fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        //println!("{:?}", _server_public_key.to_string());
        Ok(true)
    }
}

pub async fn connect(auth: ConnectRequest) -> Result<(), String> {
    let config = Arc::new(Config::default());
    let client = Client {};

    let mut handle = russh::client::connect(config, format!("{}:{}", auth.ip, auth.port), client)
        .await
        .map_err(|e| format!("failed to connect to SSH server: {}", e))?;

    let res = handle
        .authenticate_password(auth.username, auth.cred)
        .await
        .map_err(|e| format!("failed to authenticate with SSH server: {}", e))?;

    if !res.success() {
        return Err("authentication failed: wrong credentials".to_string());
    }

    Ok(())
}

use russh::client::{Config, Handler};
use std::sync::Arc;

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
        println!("{:?}", _server_public_key.to_string());
        Ok(true)
    }
}

pub async fn login() {
    let config = Arc::new(Config::default());
    let client = Client {};

    let mut handle = russh::client::connect(config, "192.168.64.2:22", client)
        .await
        .expect("error connecting to ssh server");

    let res = handle
        .authenticate_password("ankit", "ankit")
        .await
        .expect("error authenticating");

    if !res.success() {
        panic!("wrong credentials")
    }

    let mut channel = handle
        .channel_open_session()
        .await
        .expect("error opening a channel");

    channel
        .exec(true, "whoami")
        .await
        .expect("error executing command");
    

    while let Some(msg) = channel.wait().await {
        match msg {
            russh::ChannelMsg::Data { data } => {
                println!("{:?}", String::from_utf8_lossy(&data));
            }
            russh::ChannelMsg::ExitStatus { exit_status } => {
                println!("Error code: {:?}", exit_status);
            }
            _ => {}
        }
    }
}

/*
connect to server successfully
Once I am back learn tokio a bit more
*/

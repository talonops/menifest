use russh::*;
use russh::client::*;
use std::sync::Arc;

// 1. Your "instruction sheet" — the Handler
struct Client;

impl client::Handler for Client {
    type Error = russh::Error;

    // Trust any server key. DO NOT do this in production.
    async fn check_server_key(
        &mut self,
        _key: &russh::keys::ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2. Config — just use defaults
    let config = Arc::new(client::Config::default());

    // 3. Dial the VM
    let mut session = client::connect(config, ("192.168.1.50", 22), Client).await?;

    // 4. Log in with password
    let auth = session.authenticate_password("your_user", "your_password").await?;
    if !auth.success() {
        panic!("auth failed");
    }

    // 5. Open a channel and run a command
    let mut channel = session.channel_open_session().await?;
    channel.exec(true, "ls -la").await?;

    // 6. Read whatever the server sends back
    while let Some(msg) = channel.wait().await {
        match msg {
            ChannelMsg::Data { ref data } => {
                print!("{}", String::from_utf8_lossy(data));
            }
            ChannelMsg::ExitStatus { exit_status } => {
                println!("\nExit code: {}", exit_status);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use async_trait::async_trait;
use maelstrom::{protocol::Message, Node, Result, Runtime};

struct EchoNode;

#[async_trait]
impl Node for EchoNode {
    async fn process(&self, runtime: Runtime, request: Message) -> Result<()> {
        let req_type = request.get_type();
        match req_type {
            "echo" => handle_echo(runtime, request).await,
            "init" => Ok(()),
            _ => {
                unreachable!("EchoNode: got type: '{}', which is not supported", req_type)
            }
        }
    }
}

async fn handle_echo(runtime: Runtime, mut req: Message) -> Result<()> {
    req.body.typ = "echo_ok".to_string();
    let body = req.body.clone();
    runtime.reply(req, body).await
}

fn main() {
    Runtime::init(async_main());
}

async fn async_main() {
    let node = EchoNode;
    let handler = Arc::new(node);
    let _ = Runtime::new().with_handler(handler).run().await;
}

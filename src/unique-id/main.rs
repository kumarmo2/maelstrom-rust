use std::sync::Arc;

use async_trait::async_trait;
use maelstrom::{protocol::Message, Node, Result, Runtime};
use serde::Serialize;
use ulid::Ulid;

#[derive(Serialize)]
struct GenerateResult {
    id: String,
}

struct UniqueNode;

#[async_trait]
impl Node for UniqueNode {
    async fn process(&self, runtime: Runtime, request: Message) -> Result<()> {
        let typ = request.get_type();
        match typ {
            "init" => Ok(()),
            "generate" => handle_generate(runtime, request).await,
            typ => {
                unreachable!("got request of type: '{}', which is not supported", typ)
            }
        }
    }
}

async fn handle_generate(runtime: Runtime, request: Message) -> Result<()> {
    let body = GenerateResult {
        id: Ulid::new().to_string(),
    };
    runtime.reply(request, body).await
}

fn main() {
    Runtime::init(async_main());
}

async fn async_main() {
    let node = UniqueNode;
    let handler = Arc::new(node);
    let _ = Runtime::new().with_handler(handler).run().await;
}

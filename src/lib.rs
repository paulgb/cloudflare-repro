use worker::{
    durable_object, event, Env, Request, Response, Result, RouteContext, Router,
    WebSocketPair,
};

const DURABLE_OBJECT: &str = "MY_DO";

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let router = Router::new();

    let response = router
        .get_async("/", forward_to_durable_object)
        .run(req, env)
        .await?;

    Ok(response)
}

async fn forward_to_durable_object(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let durable_object = ctx.env.durable_object(DURABLE_OBJECT)?;
    let stub = durable_object.id_from_name(&"")?.get_stub()?;
    stub.fetch_with_request(req).await
}

#[durable_object]
pub struct MyDo;

#[durable_object]
impl DurableObject for MyDo {
    fn new(state: State, _env: Env) -> Self {
        Self
    }

    async fn fetch(&mut self, req: Request) -> Result<Response> {
        req.headers()
            .get("Upgrade")
            .expect("Upgrade header not set");

        let WebSocketPair { client, server } = WebSocketPair::new()?;

        server.accept()?;

        server.send_with_bytes([1, 2, 3])?;

        let resp = Response::from_websocket(client)?;
        Ok(resp)
    }
}

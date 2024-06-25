use worker::*;
use worker_bindings::bindings;

/* This knows all your bindings in wrangler.toml */
#[bindings]
struct Bindings;

#[event(fetch)]
pub async fn main(_req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    /* load bindings from env */
    let b = Bindings::from(&env);

    let _data = b.KV.get("data").text().await
        .expect("Failed to get data");

    Response::ok("Hello, worker-bindings!")
}

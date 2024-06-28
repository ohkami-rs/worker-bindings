use worker::*;
use worker_bindings::bindings;

/* This knows all your bindings in wrangler.toml */
#[bindings]
struct Bindings;

#[event(fetch)]
pub async fn main(_req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    /* load bindings from env */
    let b = Bindings::from(&env);

    let _var: &'static str = Bindings::VAR;

    let _data = b.KV.get("data").text().await?;

    Response::ok("Hello, worker-bindings!")
}

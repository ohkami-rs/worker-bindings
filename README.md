<h1 align="center">worker-bindings</h1>
<p align="center">Automatically detect <em>bindings</em> in <code>wrangler.toml</code> and bind them to a Rust struct</p>

<div align="right">
    <a href="https://github.com/ohkami-rs/worker-bindings/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/crates/l/worker-bindings.svg" /></a>
    <a href="https://github.com/ohkami-rs/worker-bindings/actions"><img alt="CI status" src="https://github.com/ohkami-rs/worker-bindings/actions/workflows/CI.yaml/badge.svg"/></a>
    <a href="https://crates.io/crates/worker-bindings"><img alt="crates.io" src="https://img.shields.io/crates/v/worker-bindings" /></a>
</div>

## Example

*wrangler.toml*
```toml
[vars]
MY_VAR = "my-variable"

[[kv_namespaces]]
binding = "MY_KV"
id      = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
```

*lib.rs*
```rust
use worker::*;
use worker_bindings::bindings;

/* This knows all your bindings in wrangler.toml */
#[bindings]
struct Bindings;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    /* load bindings from env */
    let b = Bindings::from(&env);

    let var: &'static str = b.MY_VAR;

    let data = b.MY_KV.get("data").text().await?;

    //...
}
```

## Note

- `#[bindings]` works in a cargo workspace but has a limitation that it can't resolve `wrangler.toml` if **more than one** members have `wrangler.toml`s.
- This crate is originally developed in [Ohkami](https://crates.io/crates/ohkami) web framework and later extracted as an independent edition.

## License
`worker-bindings` is licensed under the MIT License ([LICENSE](https://github.com/ohkami-rs/worker-bindings/blob/main/LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)).
mod internal;


/// Automatically bind bindings in your `wrangler.toml` into a Rust struct
/// 
/// - This uses the default (top-level) env by default. You can configure it
///   by passing an env name as argument like `#[bindings(dev)]`
/// - You can the bindings instance by `＜struct name＞::from(&env)`.
/// 
/// <br>
/// 
/// ## Example
/// ---
/// *wrangler.toml*
/// ```ignore
/// [[kv_namespaces]]
/// binding = "MY_KV"
/// id      = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
/// ```
/// ---
/// *lib.rs*
/// ```ignore
/// use worker::*;
/// use worker_bindings::bindings;
/// 
/// #[bindings]
/// struct Bindings;
/// 
/// #[event(fetch)]
/// pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
///     let b = Bindings::from(&env);
/// 
///     let data = b.MY_KV.get("data").text().await
///         .expect("Failed to get data");
/// 
///     //...
/// }
/// ```
/// ---
/// 
/// <br>
/// 
/// _**note**_ : `#[bindings]` supports
/// 
/// - KV
/// - D1
/// - Vars
/// - Service
/// - Queue (producer)
/// - Durable Objects
/// 
/// in cuurent version, as `worker` crate does.
/// ( `worker` supports secrets, but secrets aren't written in wrangler.toml... )
/// 
/// <br>
/// 
/// _**tips**_ :
/// 
/// - You can switch between multiple `env`s by feature flags like `#[cfg_attr(feature = "...", bindings(env_name))]`
/// - For `rust-analyzer` user : when editting wrangler.toml around bindings,
///   you'll need to reload `#[bindings] struct ...;` to notice the new bindings to rust-analyer.
///   For that, what you have to do is just **deleting `;` and immediate restoring it**.
#[proc_macro_attribute]
pub fn bindings(env: proc_macro::TokenStream, bindings_struct: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::bindings(env.into(), bindings_struct.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

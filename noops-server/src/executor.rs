use host::{self, WasiCtx};
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store, WasmBacktraceDetails,
};

use crate::bindgen;

pub async fn execute(
    wasm: Vec<u8>,
    request: bindgen::Request<'_>,
) -> anyhow::Result<bindgen::Response> {
    println!("Starting");
    println!("Creating Config");
    let mut config = Config::new();
    config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);

    println!("Creating Engine");
    let engine = Engine::new(&config)?;

    println!("Creating Component");
    let component = Component::from_binary(&engine, &wasm)?;

    println!("Creating Linker");
    let mut linker = Linker::new(&engine);

    host::command::add_to_linker(&mut linker, |ctx: &mut WasiCtx| ctx)?;

    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let mut store = Store::new(
        &engine,
        WasiCtxBuilder::new()
            .inherit_stdin()
            .inherit_stdout()
            .build(),
    );

    let (bindings, _) =
        bindgen::Handler::instantiate_async(&mut store, &component, &linker).await?;

    let response = bindings.call_handle(&mut store, request).await?;
    Ok(response)
}

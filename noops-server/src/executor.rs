use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

pub fn execute(wasm: Vec<u8>) -> anyhow::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_binary(&engine, &wasm)?;
    linker.module(&mut store, "hello_world", &module)?;
    linker
        .get_default(&mut store, "hello_world")?
        .typed::<(), ()>(&store)?
        .call(&mut store, ())?;

    Ok(())
}

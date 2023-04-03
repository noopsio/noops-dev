use anyhow;
use std::fs;
use wasmtime;
use wit_component::ComponentEncoder;

static ADAPTER_PATH: &str = "./wit/wasi_snapshot_preview1.wasm";
static ADAPTER_NAME: &str = "wasi_snapshot_preview1";

wasmtime::component::bindgen!({
    world: "handler",
    async: true
});

pub fn create_component(wasm_module: &[u8]) -> anyhow::Result<Vec<u8>> {
    let adapter = fs::read(ADAPTER_PATH)?;

    let component = ComponentEncoder::default()
        .adapter(ADAPTER_NAME, &adapter)?
        .module(&wasm_module)?
        .encode()?;
    Ok(component)
}

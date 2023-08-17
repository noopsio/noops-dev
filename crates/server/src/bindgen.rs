use std::fs;
use wasmtime::component::bindgen;
use wit_component::ComponentEncoder;

const ADAPTER_PATH: &str = "../../crates/wit/wasi_snapshot_preview1.wasm";
const ADAPTER_NAME: &str = "wasi_snapshot_preview1";

bindgen!({
    world: "handler",
    path: "../wit",
    async: true
});

#[allow(clippy::derivable_impls)]
impl Default for Request<'_> {
    fn default() -> Self {
        Self {
            query_params: Default::default(),
        }
    }
}

pub fn create_component(wasm_module: &[u8]) -> anyhow::Result<Vec<u8>> {
    let adapter = fs::read(ADAPTER_PATH)?;
    let component = ComponentEncoder::default()
        .module(wasm_module)?
        .adapter(ADAPTER_NAME, &adapter)?
        .encode()?;
    Ok(component)
}

use std::fs;
use wasmtime::component::bindgen;
use wit_component::ComponentEncoder;

const ADAPTER_PATH: &str = "../wit/wasi_snapshot_preview1.reactor.wasm";
const ADAPTER_NAME: &str = "wasi_snapshot_preview1";

lazy_static::lazy_static! {
    static ref ADAPTER: Vec<u8> = fs::read(ADAPTER_PATH).unwrap();
}

bindgen!({
    world: "handler",
    path: "../wit",
    async: true
});

#[allow(clippy::derivable_impls)]
impl Default for Request {
    fn default() -> Self {
        Self {
            query_params: Default::default(),
        }
    }
}

pub fn create_component(wasm_module: &[u8]) -> anyhow::Result<Vec<u8>> {
    let component = ComponentEncoder::default()
        .module(wasm_module)?
        .adapter(ADAPTER_NAME, &ADAPTER)?
        .validate(true)
        .encode()?;
    Ok(component)
}

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
    let config = create_config();
    let engine = Engine::new(&config)?;
    let component = Component::from_binary(&engine, &wasm)?;

    let mut linker = Linker::new(&engine);
    host::command::add_to_linker(&mut linker, |ctx: &mut WasiCtx| ctx)?;
    let linker = linker;

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

fn create_config() -> Config {
    Config::new()
        .wasm_backtrace_details(WasmBacktraceDetails::Enable)
        .wasm_component_model(true)
        .async_support(true)
        .clone()
}

#[cfg(test)]
mod tests {
    use crate::bindgen;
    use crate::executor;

    #[tokio::test]
    async fn return_status_code() -> anyhow::Result<()> {
        let path = env!("CARGO_CDYLIB_FILE_RETURN_STATUS_CODE_200");
        let module = std::fs::read(path).expect("Unable to read module");
        let component =
            bindgen::create_component(&module).expect("Unable to create component from module");
        let request = bindgen::Request::default();
        let response = executor::execute(component, request).await?;

        assert_eq!(200, response.status);
        Ok(())
    }

    #[tokio::test]
    async fn return_params() -> anyhow::Result<()> {
        let path = env!("CARGO_CDYLIB_FILE_RETURN_PARAMS");
        println!("path {}", path);
        let module = std::fs::read(path).expect("Unable to read module");
        let component =
            bindgen::create_component(&module).expect("Unable to create component from module");
        let request = bindgen::Request {
            query_params: &[("key1", "value1"), ("key2", "value2"), ("key3", "value3")],
        };
        let response = executor::execute(component, request).await?;
        assert_eq!(200, response.status);
        assert_eq!(
            format!("key1=value1\nkey2=value2\nkey3=value3\n"),
            response.body
        );
        Ok(())
    }
}

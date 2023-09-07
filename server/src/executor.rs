use crate::bindgen;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store, WasmBacktraceDetails,
};
use wasmtime_wasi::preview2::{self, Table, WasiCtx, WasiCtxBuilder, WasiView};

lazy_static::lazy_static! {
    static ref ENGINE: Engine = {
        let mut config = Config::new();
        config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
        config.wasm_component_model(true);
        config.async_support(true);

        Engine::new(&config).unwrap()
    };
}

struct CommandCtx {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for CommandCtx {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub async fn execute(
    wasm: Vec<u8>,
    request: bindgen::Request,
) -> anyhow::Result<bindgen::Response> {
    let component = Component::from_binary(&ENGINE, &wasm)?;

    let mut linker = Linker::new(&ENGINE);
    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().build(&mut table)?;
    preview2::command::add_to_linker(&mut linker)?;

    let linker = linker;
    let mut store = Store::new(&ENGINE, CommandCtx { table, wasi });

    let (bindings, _) =
        bindgen::Handler::instantiate_async(&mut store, &component, &linker).await?;

    let response = bindings.call_handle(&mut store, &request).await?;
    Ok(response)
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
        let module = std::fs::read(path).expect("Unable to read module");
        let component =
            bindgen::create_component(&module).expect("Unable to create component from module");
        let request = bindgen::Request {
            query_params: vec![
                ("key1".to_string(), "value1".to_string()),
                ("key2".to_string(), "value2".to_string()),
                ("key3".to_string(), "value3".to_string()),
            ],
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

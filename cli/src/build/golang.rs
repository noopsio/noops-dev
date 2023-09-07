use super::BaseAdapter;
use anyhow::{Context, Result};
use std::{borrow::Cow, fs, path::Path};
use wasm_encoder::{Encode, Section};
use wit_component::{self, StringEncoding};
use wit_parser::{PackageId, Resolve, UnresolvedPackage};

const PROGRAM: &str = "tinygo";
const DEFAULT_WORLD: &str = "handler";

pub struct GolangAdapter {
    adapter: BaseAdapter,
}

impl GolangAdapter {
    pub fn new() -> Self {
        GolangAdapter {
            adapter: BaseAdapter {
                program: PROGRAM.to_string(),
            },
        }
    }

    pub fn build(&self, work_dir: &Path) -> anyhow::Result<()> {
        let out_path = work_dir.join("out");
        if !out_path.exists() {
            fs::create_dir(&out_path)?;
        }
        let command = self.adapter.build_command(
            work_dir,
            &["build", "-target", "wasi", "-o", "./out/handler.wasm"],
        );
        self.adapter.execute(command)?;
        let handler_path = out_path.join("handler.wasm");
        let wit_path = work_dir.join("wit").join("handler.wit");

        let wasm =
            fs::read(&handler_path).context(format!("failed to read handler {handler_path:?}"))?;
        let wasm = self.embed_component(wasm, &wit_path)?;
        fs::write(handler_path, wasm)?;
        Ok(())
    }

    fn embed_component(&self, mut wasm: Vec<u8>, wit_path: &Path) -> Result<Vec<u8>> {
        let (resolve, id) = parse_wit(wit_path)?;
        let world = resolve.select_world(id, Some(DEFAULT_WORLD))?;
        let encoded = wit_component::metadata::encode(&resolve, world, StringEncoding::UTF8, None)?;
        let section = wasm_encoder::CustomSection {
            name: "component-type".into(),
            data: Cow::Borrowed(&encoded),
        };
        wasm.push(section.id());
        section.encode(&mut wasm);
        Ok(wasm)
    }
}

fn parse_wit(path: &Path) -> Result<(Resolve, PackageId)> {
    let mut resolve = Resolve::default();
    let wit_content =
        fs::read_to_string(path).context(format!("failed to read wit file {path:?}"))?;
    let pkg = UnresolvedPackage::parse(path, &wit_content)?;
    let id = resolve.push(pkg)?;
    Ok((resolve, id))
}

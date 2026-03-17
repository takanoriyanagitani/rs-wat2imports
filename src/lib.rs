use std::io;

use std::fmt::Write;

use std::io::BufWriter;
use std::io::Write as IoWrite;

use std::io::Read;

use wasmi::Engine;
use wasmi::ExternType;
use wasmi::Module;

pub struct Runtime(pub Engine);

impl Runtime {
    pub fn create_module(&self, wasm_or_wat: &[u8]) -> Result<Module, io::Error> {
        Module::new(&self.0, wasm_or_wat).map_err(io::Error::other)
    }
}

#[derive(serde::Serialize)]
pub struct ImportTypeDto<'a> {
    pub module: &'a str,
    pub name: &'a str,
    pub extern_type: &'a str,
}

pub struct Parsed(pub Module);

impl Parsed {
    pub fn imports2writer<W>(&self, buf: &mut String, wtr: &mut W) -> Result<(), io::Error>
    where
        W: FnMut(ImportTypeDto) -> Result<(), io::Error>,
    {
        let imports = self.0.imports();
        for ityp in imports {
            let module: &str = ityp.module();
            let name: &str = ityp.name();
            let ty: &ExternType = ityp.ty();
            buf.clear();
            write!(buf, "{ty:?}").map_err(io::Error::other)?;
            let extern_type: &str = buf;
            let dto = ImportTypeDto {
                module,
                name,
                extern_type,
            };
            wtr(dto)?;
        }
        Ok(())
    }

    pub fn imports2jsons2stdout(&self, buf: &mut String) -> Result<(), io::Error> {
        let o = io::stdout();
        let mut ol = o.lock();
        let mut bw = BufWriter::new(&mut ol);
        let mut wtr = |dto: ImportTypeDto| {
            serde_json::to_writer(&mut bw, &dto)?;
            writeln!(&mut bw)
        };
        self.imports2writer(buf, &mut wtr)?;
        bw.flush()?;
        drop(bw);
        ol.flush()
    }
}

pub fn reader2bytes_limited<R>(rdr: R, limit: u64) -> Result<Vec<u8>, io::Error>
where
    R: Read,
{
    let mut taken = rdr.take(limit);
    let mut buf: Vec<u8> = vec![];
    taken.read_to_end(&mut buf)?;
    Ok(buf)
}

pub const WASM_OR_WAT_SIZE_LIMIT_DEFAULT: u64 = 16777216;

pub fn stdin2bytes2engine2module2imports2jsons2stdout(limit: u64) -> Result<(), io::Error> {
    let wasm_or_wat: Vec<u8> = reader2bytes_limited(io::stdin().lock(), limit)?;
    let eng = Engine::default();
    let module: Module = Runtime(eng).create_module(&wasm_or_wat)?;
    let mut buf: String = String::new();
    Parsed(module).imports2jsons2stdout(&mut buf)?;
    Ok(())
}

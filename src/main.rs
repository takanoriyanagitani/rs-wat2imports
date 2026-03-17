use std::io;
use std::process::ExitCode;

use rs_wat2imports::stdin2bytes2engine2module2imports2jsons2stdout;

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<u64, io::Error> {
    let first = match args.next() {
        None => return Ok(rs_wat2imports::WASM_OR_WAT_SIZE_LIMIT_DEFAULT),
        Some(arg) => arg,
    };

    if first != "--input-size-limit" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Unknown argument: {first}"),
        ));
    }

    let val_str = args.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing value for --input-size-limit",
        )
    })?;

    let limit = val_str.parse().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Invalid value for --input-size-limit: {e}"),
        )
    })?;

    if let Some(extra) = args.next() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Unexpected extra argument: {extra}"),
        ));
    }

    Ok(limit)
}

fn sub() -> Result<(), io::Error> {
    let limit = parse_args(std::env::args().skip(1))?;
    stdin2bytes2engine2module2imports2jsons2stdout(limit)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}

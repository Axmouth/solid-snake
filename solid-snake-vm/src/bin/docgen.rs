use std::{fs::File, io::Write};

use solid_snake_vm::{docs::Docs, opcodes::OpCode};

fn main() -> std::io::Result<()> {
    let docs = Docs {
        instructions: OpCode::get_docs(),
    };

    let mut file = File::create("INSTRUCTIONS.md")?;
    file.write_all(docs.to_markdown().as_bytes())?;
    Ok(())
}

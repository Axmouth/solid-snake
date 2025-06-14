use std::{fs::{self, File}, io::Write};

use solid_snake_vm::{docs::Docs, opcodes::OpCode};

fn main() -> std::io::Result<()> {
    let docs = Docs {
        instructions: OpCode::get_docs(),
    };

    let mut md_file = File::create("./solid-snake-vm/INSTRUCTIONS.md")?;
    md_file.write_all(docs.to_markdown().as_bytes())?;

    let mut json_file = File::create("./solid-snake-vm/docs.json")?;
    json_file.write_all(docs.to_json().as_bytes())?;

    println!("Documentation generated successfully in INSTRUCTIONS.md and docs.json");

    let template = fs::read_to_string("./solid-snake-vm/docs.template.html").expect("Failed to read template.html");
    let docs = fs::read_to_string("./solid-snake-vm/docs.json").expect("Failed to read docs.json");

    // Escape closing script tags to prevent breaking out of the tag
    let safe_json = docs.replace("</script>", "<\\/script>");

    let output = template.replace("__DOCS_PLACEHOLDER__", &safe_json);
    fs::write("./solid-snake-vm/docs.index.html", output).expect("Failed to write docs.index.html");

    println!("✅ Generated docs.index.html");

    Ok(())
}

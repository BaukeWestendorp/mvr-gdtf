use std::{
    fs::write,
    io::Write as _,
    process::{Command, Output, Stdio},
};

use quote::ToTokens;
use xsd_parser::{
    Config, MetaTypes, Schemas,
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::default().with_serde_quick_xml();
    config.parser.resolver = vec![Resolver::File];
    config.parser.flags = ParserFlags::all();
    config.parser.schemas = vec![Schema::File("codegen/schema/mvr.xsd".into())];
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.optimizer.flags = OptimizerFlags::all();
    config.generator.flags = GeneratorFlags::all();

    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = rename_types(&schemas, meta_types)?;
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;
    let mut code = module.to_token_stream().to_string();

    code = rustfmt_pretty_print(code)?;

    write("mvr-gdtf/src/mvr/schema.rs", code)?;

    Ok(())
}

fn rename_types(
    _schemas: &Schemas,
    mut types: MetaTypes,
) -> Result<MetaTypes, Box<dyn std::error::Error>> {
    for (ident, ty) in types.items.iter_mut() {
        let exclude = ["GeneralSceneDescription"];
        if exclude.contains(&ident.name.as_str()) {
            continue;
        }

        ty.display_name = Some(ident.name.to_string());
    }

    Ok(types)
}

pub fn rustfmt_pretty_print(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();

    write!(stdin, "{code}")?;
    stdin.flush()?;
    drop(stdin);

    let Output { status, stdout, stderr } = child.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&stdout);
    let stderr = String::from_utf8_lossy(&stderr);

    if !status.success() {
        let code = status.code();
        match code {
            Some(code) => {
                if code != 0 {
                    panic!("The `rustfmt` command failed with return code {code}!\n{stderr}");
                }
            }
            None => {
                panic!("The `rustfmt` command failed!\n{stderr}")
            }
        }
    }

    Ok(stdout.into())
}

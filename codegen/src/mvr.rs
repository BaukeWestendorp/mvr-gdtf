use quote::ToTokens as _;
use xsd_parser::*;

use crate::common::*;

pub fn generate_mvr_schema() -> anyhow::Result<()> {
    let mut config = default_config();
    configure_parser(&mut config, "codegen/schema/mvr.xsd");
    configure_interpreter(&mut config);
    configure_optimizer(&mut config);
    configure_generator(&mut config);

    let mut schemas = exec_parser(config.parser)?;
    rename_schema_content(&mut schemas, &[("GeneralSceneDescription", "GeneralSceneDescription")]);

    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;

    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let data_types = modify_data_types(&schemas, data_types)?;

    let module = exec_render(config.renderer, &data_types)?;

    let mut code = module.to_token_stream().to_string();
    code = rustfmt_pretty_print(code)?;
    std::fs::write("codegen/schema/mvr.rs", code)?;

    Ok(())
}

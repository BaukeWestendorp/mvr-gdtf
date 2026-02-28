use anyhow::{Context, Result};
use std::{
    io::Write as _,
    process::{Command, Output, Stdio},
};
use xsd_parser::{
    Config, DataTypes, Schemas,
    config::{
        GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver, Schema,
        TypePostfix,
    },
    models::schema::{
        SchemaId,
        xs::{
            AttributeGroupType, AttributeType, ComplexBaseType, ElementType, GroupType, NS_XS,
            SchemaContent, SimpleBaseType,
        },
    },
};

pub fn default_config() -> Config {
    Config::default().with_serde_quick_xml()
}

pub fn configure_parser(config: &mut Config, xsd_path: &str) {
    config.parser.resolver = vec![Resolver::File];
    config.parser.flags = ParserFlags::all();
    config.parser.schemas = vec![Schema::File(xsd_path.into())];
}

pub fn configure_interpreter(config: &mut Config) {
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
}

pub fn configure_optimizer(config: &mut Config) {
    config.optimizer.flags = OptimizerFlags::all();
}

pub fn configure_generator(config: &mut Config) {
    config.generator.type_postfix = TypePostfix {
        type_: "".to_string(),
        element: "".to_string(),
        element_type: "".to_string(),
        nillable_content: "".to_string(),
        dynamic_element: "".to_string(),
    };
    config.generator.flags = GeneratorFlags::all()
        - GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS
        - GeneratorFlags::ABSOLUTE_PATHS_INSTEAD_USINGS;
}

pub fn rename_schema_content(schemas: &mut Schemas, renaming_rules: &[(&'static str, &str)]) {
    for (old_name, new_name) in renaming_rules {
        for content in schemas.get_schema_mut(&SchemaId(0)).unwrap().schema.content.iter_mut() {
            match content {
                SchemaContent::SimpleType(SimpleBaseType { name, .. })
                | SchemaContent::ComplexType(ComplexBaseType { name, .. })
                | SchemaContent::Group(GroupType { name, .. })
                | SchemaContent::AttributeGroup(AttributeGroupType { name, .. })
                | SchemaContent::Element(ElementType { name, .. })
                | SchemaContent::Attribute(AttributeType { name, .. }) => {
                    if name.as_deref() == Some(old_name) {
                        *name = Some((*new_name).into());
                    }
                }
                _ => {}
            };
        }
    }
}

pub fn modify_data_types<'a>(
    schemas: &'a Schemas,
    mut data_types: DataTypes<'a>,
) -> Result<DataTypes<'a>> {
    let namespace_id = schemas.resolve_namespace(&Some(NS_XS)).unwrap();
    data_types.items.retain(|ident, _item| ident.ns != Some(namespace_id));
    Ok(data_types)
}

pub fn rustfmt_pretty_print(code: String) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn rustfmt process")?;

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
                    anyhow::bail!(
                        "The `rustfmt` command failed with return code {code}!\n{stderr}"
                    );
                }
            }
            None => {
                anyhow::bail!("The `rustfmt` command failed!\n{stderr}")
            }
        }
    }

    Ok(stdout.into())
}

use protobuf;
use serde_yaml;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub struct GeneratorContext<'a> {
    pub proto_file: &'a [protobuf::descriptor::FileDescriptorProto],
    pub file_to_generate: &'a [PathBuf],
    pub parameter: &'a str,
    pub compiler_version: &'a protobuf::plugin::Version,
}

pub type GeneratorResult = Result<Vec<protobuf::plugin::CodeGeneratorResponse_File>, String>;

pub trait CodeGenerator {
    fn name(&self) -> &'static str;
    fn parse_configure(&mut self);
    fn generate(&self, ctx: &mut GeneratorContext) -> GeneratorResult;
    fn get_support_features(&self) -> u64;
}

pub type CodeGeneratorSet = Vec<Box<dyn CodeGenerator>>;

// fn create_generators_from_yaml_map()

pub fn create_generators_from_configure_file(
    file_path: &std::path::PathBuf,
) -> Result<CodeGeneratorSet, String> {
    // TODO
    let conf_file: File = OpenOptions::new()
        .read(true)
        .open(file_path)
        .map_err(|err| err.to_string())?;

    let conf_value: serde_yaml::Value =
        serde_yaml::from_reader(conf_file).map_err(|err| err.to_string())?;

    // match conf_value {
    //     Sequence(ele) => {},
    //     Mapping(ele) =>
    // }
    Ok(Vec::new())
}

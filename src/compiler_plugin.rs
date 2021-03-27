// use protobuf::descriptor::FileDescriptorProto;
use protobuf::plugin::*;
use protobuf::Message;
use std::io::stdin;
use std::io::stdout;
use std::path::PathBuf;

use super::generator::*;

pub struct PluginParameter {
    pub configure_file: PathBuf,
    request: CodeGeneratorRequest,
}

pub fn plugin_parse() -> Result<PluginParameter, String> {
    let req =
        CodeGeneratorRequest::parse_from_reader(&mut stdin()).map_err(|err| err.to_string())?;

    Ok(PluginParameter {
        configure_file: req.get_parameter().clone().into(),
        request: req,
    })
}

pub fn plugin_fatal_error(error_message: String) {
    let mut resp = CodeGeneratorResponse::new();
    resp.set_error(error_message);
    resp.write_to_writer(&mut stdout()).unwrap();
}

pub fn plugin_main(params: &PluginParameter, gens: &CodeGeneratorSet) {
    let mut ctx = GeneratorContext {
        proto_file: &params.request.proto_file,
        file_to_generate: &params
            .request
            .file_to_generate
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<_>>(),
        parameter: params.request.get_parameter(),
        compiler_version: params.request.get_compiler_version(),
    };
    let mut resp = CodeGeneratorResponse::new();
    for generator in gens {
        let generator_feature = generator.get_support_features();
        let error_message = match generator.generate(&mut ctx) {
            Ok(res) => {
                resp.set_supported_features(resp.get_supported_features() | generator_feature);
                res.iter().for_each(|file| {
                    resp.mut_file().push(file.clone());
                });
                String::default()
            }
            Err(error_message) => error_message,
        };
        if !error_message.is_empty() {
            resp.set_error(format!(
                "{}============ Generator {} has error ============\n{}\n",
                resp.get_error(),
                generator.name(),
                error_message
            ));
        }
    }

    resp.write_to_writer(&mut stdout()).unwrap();
}

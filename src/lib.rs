
extern crate protobuf;
extern crate serde;
extern crate serde_yaml;

mod compiler_plugin;
pub mod generator;

pub fn protoc_ext_render_main() {
    let params = match compiler_plugin::plugin_parse() {
        Ok(x) => x,
        Err(e) => {
            compiler_plugin::plugin_fatal_error(e);
            return;
        }
    };

    let generators = match generator::create_generators_from_configure_file(&params.configure_file) {
        Ok(x) => x,
        Err(e) => {
            compiler_plugin::plugin_fatal_error(e);
            return;
        }
    };

    compiler_plugin::plugin_main(&params, &generators);
}

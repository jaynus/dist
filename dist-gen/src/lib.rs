#[path = "../../schema/reflection_generated.rs"]
mod _fbs_reflection;
mod fbs {
    pub use crate::_fbs_reflection::reflection as reflection;
}

use flatc_rust;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::ffi::OsString;

pub fn compile_components() -> std::io::Result<()> {
    let in_dir = std::fs::read_dir("./schema/components/")?;
    let out_dir = Path::new("schema/generated/");

    // Make the generated dir
    std::fs::create_dir_all(out_dir)?;

    let mut builder = ComponentBuilder::new(out_dir);

    for file in in_dir {
        // TODO: Reflect the component
        let path = &file?.path();

        builder = builder.compile_component(path)?;
    }

    builder.finish()
}

struct ComponentBuilder {
    output_directory: PathBuf,
    components: Vec<OsString>,
    input_files: Vec<PathBuf>,
}
impl ComponentBuilder {
    pub fn new(output_directory: &Path) -> Self {
        Self {
            components: Vec::new(),
            input_files: Vec::new(),
            output_directory: output_directory.to_path_buf(),
        }
    }
}
impl ComponentBuilder {
    pub fn finish<'a>(&self) -> std::io::Result<()> {
        use std::io::{Read, Write};

        let mut include_file = String::new();
        include_file.push_str("pub mod components {\n");
        include_file.push_str("use std::mem;\n");
        include_file.push_str("use std::cmp::Ordering;\n");
        include_file.push_str("use flatbuffers::EndianScalar;\n\n");

        include_file.push_str("include!(\"../../../schema/base_component_generated.rs\");\n\n");

        //
        // Reflect each file and make sure its valid, and index the reflection
        //

        let mut root_names = HashMap::<OsString, String>::new();
        for component in self.components.iter() {
            let mut bfbs = component.clone();
            bfbs.push(".bfbs");

            let path = self.output_directory.join(bfbs.to_str().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to create include file"))?);

            // Reflect it
            let mut f = std::fs::File::open(path)?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf).expect("file reading failed");
            let reflected = fbs::reflection::get_root_as_schema(&buf[..]);
            root_names.insert(component.clone(),
                              reflected.root_table().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to create include file"))?.name().to_string());
        }

        //
        // Generate includes
        //

        for component in self.components.iter() {
            let mut file = component.clone();
            file.push("_generated.rs");
            include_file.push_str(&format!("include!(\"{}\");\n", file.to_str()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to create include file"))?));
        }
        include_file.push_str("\n");

        //
        // Generate types enums
        //
        let mut type_id_enum = String::new();
        let mut type_enum = String::new();

        type_enum.push_str("pub enum Types<'a> { \n");
        type_id_enum.push_str("pub enum TypeIds { \n");
        for (i, component) in self.components.iter().enumerate() {
            let str_name = &root_names[component];
            type_enum.push_str(&format!("    {}({}<'a>),\n", str_name, str_name));
            type_id_enum.push_str(&format!("    {} = {},\n", str_name, i));
        }
        type_enum.push_str("    Unknown,\n}\n");
        type_id_enum.push_str("    Unknown,\n}\n");

        include_file.push_str(&type_enum);
        include_file.push_str(&type_id_enum);

        //
        // Generate ID based type return reader and builder
        //
        include_file.push_str("\n#[inline(always)]\npub fn parse_type<'a>(id: u64, buf: &'a [u8]) -> Types<'a> { \n");
        include_file.push_str("    match id {\n");
        for (i, component) in self.components.iter().enumerate() {
            let str_obj_name = &root_names[component];
            let get_root_name = format!("get_root_as_{}", Self::camel_to_snake_case(&root_names[component]));

            include_file.push_str(&format!("        {} => Types::{}({}(buf)),\n", i, str_obj_name, get_root_name));

        }
        include_file.push_str("        _ => Types::Unknown,\n    }\n}\n");

        // Generic parser of base_component
        include_file.push_str("\n#[inline(always)]\npub fn parse_base_component<'a>(buf: &'a [u8]) -> Types<'a> { \n");
        include_file.push_str("    let base = dist::get_root_as_base_component(buf);\n");
        include_file.push_str("    parse_type(base.id(), buf)\n");
        include_file.push_str("}\n");


        include_file.push_str("\n\n}");
        {
            let mut f = std::fs::File::create(self.output_directory.join("include.rs"))?;
            f.write_all(include_file.as_bytes())?;
        }

        Ok(())
    }

    pub fn compile_component(mut self, path: &Path) -> std::io::Result<Self>
    {
        //let out_file = self.output_directory.join(Path::new(&component_name));
        let component_name = path.file_stem().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to extract component name"))?
        .to_os_string();
        self.components.push(component_name);
        self.input_files.push(path.to_path_buf());

        flatc_rust::Flatc::from_path(Path::new("../bin/bin/flatc").to_path_buf()).run(flatc_rust::Args {
            lang: "rust",  // `rust` is the default, but let's be explicit
            inputs: &[path],
            includes: &[&Path::new("../schema")],
            binary: true,
            schema: true,
            out_dir: &self.output_directory,
            ..Default::default()
        })?;

        Ok(self)
    }

    fn camel_to_snake_case(s: &str) -> String {
        let mut result_chars: Vec<char> = Vec::new();
        let mut first_char = true;
        for c in s.chars() {
            assert!(c.is_alphanumeric(),
                    format!("non-alphanumeric character '{}', i.e. {} in identifier '{}'",
                            c, c as usize, s));
            if c.is_uppercase() && !first_char {
                result_chars.push('_');
            }
            result_chars.push(c.to_ascii_lowercase());
            first_char = false;
        }
        result_chars.into_iter().collect()
    }
}

pub mod typing {
    use std::collections::HashMap;
    use crate::fbs::reflection;

    pub fn generate_type_table(binary_repr: &[u8]) -> std::io::Result<HashMap<u64, String>> {
        let schema = reflection::get_root_as_schema(binary_repr);

        let mut counter: u64 = 0;
        let mut res = HashMap::new();

        for obj in schema.objects().iter() {
            res.insert(counter, obj.name().to_string());
            counter += 1;
        }


        Ok(res)
    }

}
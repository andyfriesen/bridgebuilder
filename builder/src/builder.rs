use std::fs::File;
use std::io::{Error, IoSlice, Write};

use proc_macro2::TokenStream;

pub struct StringBuilder {
    chunks: Vec<String>,
}

impl StringBuilder {
    pub fn new() -> Self {
        StringBuilder { chunks: Vec::new() }
    }

    pub fn add<S : Into<String>>(&mut self, s: S) {
        self.chunks.push(s.into());
    }

    pub fn write(&self, file: &mut File) -> Result<(), Error> {
        let mut ioslices = Vec::new();
        for s in &self.chunks {
            ioslices.push(IoSlice::new(s.as_bytes()))
        }

        file.write_vectored(&ioslices)?;

        Ok(())
    }
}

pub struct Builder {
    pub structs: StringBuilder,
    pub methods: StringBuilder,
    pub get_specializations: StringBuilder,
    pub rust_module: TokenStream,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            structs: StringBuilder::new(),
            methods: StringBuilder::new(),
            get_specializations: StringBuilder::new(),
            rust_module: TokenStream::new(),
        }
    }

    pub fn add_method(&mut self, cxx_type_name: &str, name: &str, variant_name: &str) {
        self.methods.add(format!(
            "extern \"C\" const {}* seedoubleplus_{}To{}(const {}* self);\n",
            cxx_type_name,
            name,
            variant_name,
            name
        ));

        self.get_specializations.add(format!(
            "template <> inline const {}* get<{}::{}>(const {}* self) {{\n    return seedoubleplus_{}To{}(self);\n}}\n\n",
            cxx_type_name,
            name,
            variant_name,
            name,
            name,
            variant_name
        ));
    }

    pub fn write(&self, filename: &str) -> Result<(), Error> {
        let mut header = File::create(filename)?;

        header.write("// This header is machine generated.  Do not edit!\n#pragma once\n\n".as_bytes())?;

        self.structs.write(&mut header)?;
        self.methods.write(&mut header)?;
        self.get_specializations.write(&mut header)?;

        Ok(())
    }
}

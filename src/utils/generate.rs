use std::fs::File;
use std::io::prelude::*;

pub struct GenerateSDL {}

impl GenerateSDL {
    pub fn export(schema: String) {
        let mut file = File::create("schema.gql").expect("Error encountered while creating file!");
        file.write_all(schema.as_bytes())
            .expect("Error while writing to file");
    }
}


pub use prelude::*;
mod prelude {
    use std::fs::File;
    use std::io::Read;

    pub use serde::Deserialize;
    pub use serde::Serialize;
    pub use crate::deserializer::Deserializer;
    pub use crate::serializer::Serializer;

    pub use crate::traits::PenguinConfig;
    pub use crate::traits::PenguinConfigGenerate;

    pub use penguin_config_derive::PenguinConfigFile;
    pub use penguin_config_derive::PenguinConfigGenerate;

    pub fn read_json(file_path: &str) -> String {
        let mut file = File::open(file_path).expect("couldn't open file");
        let mut json = String::new();
        file.read_to_string(&mut json).expect("couldn't read to json");
        json
    }
}

mod traits {
    pub trait PenguinConfig {
        fn read_config() -> Self;
    }

    pub trait PenguinConfigGenerate {
        fn generate_penguin_config_file();
    }
}

mod deserializer {
    use std::marker::PhantomData;
    use crate::read_json;

    /// let t: T = Deserializer::file_path("some_path.json").deserialize();
    pub struct Deserializer<'a, T: serde::Deserialize<'a>> {
        json: String,
        phantom_data: PhantomData<&'a T>,
    }

    impl<'a, T> Deserializer<'a, T> where T: serde::Deserialize<'a> {
        pub fn file_path(file_path: &str) -> Self {
            Self {
                json: read_json(file_path),
                phantom_data: PhantomData,
            }
        }

        pub fn deserialize(&'a self) -> T {
            let config: T = serde_json::from_str(&self.json)
                .expect(&format!("couldn't deserialize string: [{}]", &self.json));
            config
        }
    }
}

mod serializer {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    /// Serializer::file_path("some_path.json").serialize(&T::default());
    pub struct Serializer {
        path: String,
    }
    
    impl Serializer {
        pub fn file_path(file_path: &str) -> Self {

            // create folder(s) if not already present
            let path = Path::new(file_path);
            let prefix = path.parent().expect("couldn't find parent dirs");
            std::fs::create_dir_all(prefix).expect("couldn't create parent dirs");

            Self { path: file_path.to_owned(), }
        }

        pub fn serialize<T: serde::Serialize>(&self, data: &T) {
            let json = serde_json::to_string(data).expect("couldn't turn data into json");

            let mut file = File::create(&self.path)
                .expect(&format!("couldn't create file at path: {}", &self.path));

            file.write_all(json.as_bytes())
                .expect(&format!("couldn't write json to file {}", &self.path));
        }
    }
}









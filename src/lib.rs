pub use prelude::*;
mod prelude {
    use std::fs::File;
    use std::io::Read;

    #[cfg(feature = "penguin-config-derive")]
    pub use penguin_config_derive::PenguinConfigFile;

    pub use super::traits::PenguinConfig;
    pub use super::deserializer::Deserializer;

    pub use serde::Deserialize;

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
}

mod deserializer {
    use std::marker::PhantomData;
    use crate::read_json;

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
            let config: T = serde_json::from_str(&self.json).expect("couldn't deserialize");
            config
        }
    }
}


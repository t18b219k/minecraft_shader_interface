use std::collections::HashMap;

pub struct FunctionRename {
    target: HashMap<String, String>,
}

impl FunctionRename {
    fn new() -> Self {
        let mut target: HashMap<String, String> = Default::default();
        target.insert("texelFetch2D".to_string(), "texelFetch".to_string());
        target.insert("texture2D".to_string(), "texture".to_string());
        Self { target }
    }
}

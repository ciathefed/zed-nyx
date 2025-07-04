use zed_extension_api::{self as zed};

struct ApolloExtension;

impl zed::Extension for ApolloExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(ApolloExtension);

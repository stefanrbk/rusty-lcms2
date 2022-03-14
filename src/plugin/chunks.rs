use crate::plugin::error::*;
use crate::plugin::*;
use crate::Signature;

#[derive(Copy, Clone)]
pub struct LogErrorChunk {
    pub handler: LogErrorHandlerFunction,
}

impl LogErrorChunk {
    pub fn new(func: Option<LogErrorHandlerFunction>) -> Self {
        Self {
            handler: func.unwrap_or(default_log_error_handler_function),
        }
    }
}

#[derive(Clone)]
pub struct TagPluginChunk {
    pub tag: Vec<TagListItem>,
}

impl TagPluginChunk {
    pub const fn new() -> Self {
        Self {
            tag: Vec::new(),
        }
    }
    pub fn get_tag_descriptor(&self, signature: Signature) -> Option<&TagDescriptor> {
        for i in self.tag.iter() {
            if i.sig == signature {
                return Some(&i.desc);
            }
        }
        None
    }
}

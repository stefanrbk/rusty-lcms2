use crate::plugin::error::LogErrorHandlerFunction;
use crate::plugin::*;
use crate::Signature;

pub struct LogErrorChunk {
    pub handler: LogErrorHandlerFunction,
}

pub struct TagPluginChunk {
    pub tag: Vec<TagListItem>,
}

impl TagPluginChunk {
    pub fn get_tag_descriptor(&self, signature: Signature) -> Option<&TagDescriptor> {
        for i in self.tag.iter() {
            if i.sig == signature {
                return Some(&i.desc);
            }
        }
        None
    }
}

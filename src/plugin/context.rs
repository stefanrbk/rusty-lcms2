use crate::plugin::*;
use crate::plugin::error::*;
use crate::plugin::chunks::*;
use std::sync::*;

pub struct Context {
    error_handler: Arc<Mutex<LogErrorChunk>>,
    tag_plugin: Arc<Mutex<TagPluginChunk>>,
}

impl Context {
    pub fn register_tag_plugin(&mut self, plugin: Option<PluginTag>) -> bool {
        let mut chunk = self.get_tag_plugin_chunk();

        match plugin {
            None => {
                chunk.tag = Vec::new();
                return true;
            },
            Some(p) => {
                chunk.tag.push(TagListItem {
                    sig: p.signature,
                    desc: p.descriptor
                });
                return true;
            }
        }
    }
    
    pub fn get_log_error_chunk(&self) -> MutexGuard<LogErrorChunk> {
        self.error_handler.lock().unwrap()
    }
    pub fn set_log_error_handler(&mut self, func: Option<LogErrorHandlerFunction>) {
        self.error_handler.lock().unwrap().handler = match func {
            Some(f) => f,
            None => default_log_error_handler_function,
        }
    }
    pub fn get_tag_plugin_chunk(&self) -> MutexGuard<TagPluginChunk> {
        self.tag_plugin.lock().unwrap()
    }
}

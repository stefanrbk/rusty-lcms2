use crate::plugin::chunks::*;
use crate::plugin::error::*;
use crate::plugin::*;
use std::sync::*;

pub struct Context {
    user_data: Arc<Mutex<Option<Box<[u8]>>>>,
    error_handler: Arc<Mutex<Box<LogErrorChunk>>>,
    tag_plugin: Arc<Mutex<Box<TagPluginChunk>>>,
}

impl Context {
    // pub fn new(plugin: Plugin, data: Option<&'_ [u8]>) -> Self {
    //     Context {
    //         user_data: Arc::new(Mutex::new(data)),
    //         error_handler: Arc::new(Mutex::new(LogErrorChunk::new(None))),
    //         tag_plugin: Arc::new(Mutex::new(TagPluginChunk::new())),
    //     }
    // }
    pub fn init_plugins(&mut self, plugin: Plugin) -> bool {
        let mut plugin = &plugin;

        while plugin.next.is_some() {
            if plugin.magic != signatures::plugin_type::MAGIC {
                signal_error(self, ErrorCode::UnknownExtension, "Unrecognized plugin");
                return false;
            }
            if plugin.expected_version > LCMS_VERSION {
                signal_error(
                    self,
                    ErrorCode::UnknownExtension,
                    format!(
                        "plugin needs Little CMS {}, current version is {}",
                        plugin.expected_version, LCMS_VERSION
                    ).as_str(),
                );
                return false;
            }

            match plugin.data {
                PluginType::Tag { .. } => {
                    if !self.register_tag_plugin(Some(plugin)) {
                        return false;
                    }
                }
            };

            plugin = plugin.next.as_ref().unwrap();
        }

        true
    }
    pub fn unregister_plugins(&mut self) {
        self.register_tag_plugin(None);
    }
    pub fn register_tag_plugin(&mut self, plugin: Option<&Plugin>) -> bool {
        let mut chunk = self.get_tag_plugin_chunk();

        match plugin {
            None => {
                chunk.tag = Vec::new();
                return true;
            }
            Some(p) => match &p.data {
                PluginType::Tag { sig, desc } => {
                    chunk.tag.push(TagListItem {
                        sig: *sig,
                        desc: *desc,
                    });
                    return true;
                }
            },
        }
    }
    pub fn get_log_error_chunk(&self) -> MutexGuard<Box<LogErrorChunk>> {
        self.error_handler.lock().unwrap()
    }
    pub fn set_log_error_handler(&mut self, func: Option<LogErrorHandlerFunction>) {
        self.error_handler.lock().unwrap().handler = match func {
            Some(f) => f,
            None => default_log_error_handler_function,
        }
    }
    pub fn get_tag_plugin_chunk(&self) -> MutexGuard<Box<TagPluginChunk>> {
        self.tag_plugin.lock().unwrap()
    }
}

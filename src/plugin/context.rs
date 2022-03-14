use crate::plugin::chunks::*;
use crate::plugin::error::*;
use crate::plugin::*;

#[derive(Clone)]
pub struct Context {
    user_data: Option<Box<[u8]>>,
    error_handler: Box<LogErrorChunk>,
    tag_plugin: Box<TagPluginChunk>,
}

impl Context {
    pub fn new(plugin: Option<&Plugin>, data: Option<Box<[u8]>>) -> Box<Self> {
        let mut value = Box::new(Context {
            user_data: match data {
                Some(data) => Some(data),
                None => None,
            },
            error_handler: alloc_log_error_chunk(None),
            tag_plugin: alloc_tag_plugin_chunk(None),
        });
        

        if plugin.is_some() {
            value.init_plugins(plugin.unwrap());
        }

        value
    }
    pub fn init_plugins(&mut self, plugin: &Plugin) -> bool {
        let mut plugin = plugin;
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
        match plugin {
            None => {
                self.tag_plugin.tag.clear();
                return true;
            }
            Some(p) => match &p.data {
                PluginType::Tag { sig, desc } => {
                    self.tag_plugin.tag.push(TagListItem {
                        sig: *sig,
                        desc: *desc,
                    });
                    return true;
                }
            },
        }
    }
    pub fn get_user_data(&self) -> Option<&Box<[u8]>> {
        self.user_data.as_ref()
    }
    pub fn get_log_error_chunk(&self) -> &Box<LogErrorChunk> {
        &self.error_handler
    }
    pub fn set_log_error_handler(&mut self, func: Option<LogErrorHandlerFunction>) {
        self.error_handler.handler = match func {
            Some(f) => f,
            None => default_log_error_handler_function,
        }
    }
    pub fn get_tag_plugin_chunk(&self) -> &Box<TagPluginChunk> {
        &self.tag_plugin
    }
}
fn alloc_log_error_chunk(src: Option<&Context>) -> Box<LogErrorChunk> {
    Box::new(match src {
        Some(src) => (*src.error_handler).clone(),
        None => LogErrorChunk::new(None)
    })
}
fn alloc_tag_plugin_chunk(src: Option<&Context>) -> Box<TagPluginChunk> {
    Box::new(match src {
        Some(src) => (*src.tag_plugin).clone(),
        None => TagPluginChunk::new()
    })
}

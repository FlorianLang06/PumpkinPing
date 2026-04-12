mod commands;

use pumpkin_plugin_api::{Context, Plugin, PluginMetadata};

struct PingPlugin;
impl Plugin for PingPlugin {
    fn new() -> Self {
        PingPlugin
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "PumpkinPing".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: env!("CARGO_PKG_AUTHORS").split(',').map(str::to_string).collect(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
        }
    }

    fn on_load(&mut self, context: Context) -> pumpkin_plugin_api::Result<()> {
        commands::ping_command::register_command(context)?;

        Ok(())
    }

    fn on_unload(&mut self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(PingPlugin);
mod commands;
mod constants;

use pumpkin_plugin_api::{Context, Plugin, PluginMetadata, permissions};

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
            dependencies: vec![],
            permissions: vec![
                permissions::HTTP_OUTBOUND.into(),
                permissions::NETWORK_DNS.into(),
            ],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        commands::ping_command::register_command(&context)?;

        /*let debug = cfg!(feature = "debug-mode");
        if !debug {
            let metadata = pumpkin_plugin_utils::init(&context)
                .map_err(|e| format!("Initialization failed: {e}"))?;

            info!(
                "Loaded plugin '{}' v{} (Dev: {})",
                metadata.plugin_name, metadata.version, metadata.dev_name
            );
        } else {
            warn!("Plugin loaded in debug mode. Skipping update check")
        }



        check_updates(debug);*/

        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        Ok(())
    }
}

/*fn check_updates(debug: bool) {
    if debug {
        return
    }
    match pumpkin_plugin_utils::check_for_updates() {
        Ok(update) => {
            if update.update_available {
                info!(
                    "A new update is available: {}!",
                    update.latest_version.as_deref().unwrap_or("unknown")
                );
            } else {
                info!("Plugin is up to date.");
            }
        }
        Err(err) => {
            warn!("Failed to check for updates: {err}");
        }
    }
}*/

pumpkin_plugin_api::register_plugin!(PingPlugin);
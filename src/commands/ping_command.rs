use pumpkin_plugin_api::command::{Command, CommandError, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::{Context, Server};
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::text::TextComponent;

struct PingCommandExecutor;
impl CommandHandler for PingCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, _args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        match sender.as_player() {
            Some(player) => {
                let ping = player.get_ping();

                let color = if ping < 30 {
                    "a"
                } else if ping < 80 {
                    "e"
                } else {
                    "c"
                };

                let component = TextComponent::text(&format!("§7Your ping is §{}{}ms", color, ping));
                sender.send_message(component);
            }
            None => {
                sender.send_message(TextComponent::text("%cYou are not a player!"))
            }
        };

        Ok(1)
    }
}

pub fn register_command(context: Context) -> pumpkin_plugin_api::Result<()> {
    let permission = "PumpkinPing:ping";

    context.register_permission(&Permission {
        node: permission.to_string(),
        description: "Allows to show own ping".to_string(),
        default: PermissionDefault::Allow,
        children: Vec::new(),
    })?;

    context.register_command(init_command_tree(), permission);

    Ok(())
}

fn init_command_tree() -> Command {
    let names = ["ping".to_string()];
    let description = "Show the ping of a player";

    Command::new(&names, description).execute(PingCommandExecutor)
}
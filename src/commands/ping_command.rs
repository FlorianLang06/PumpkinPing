use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::permission::{Permission, PermissionDefault};
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use pumpkin_plugin_api::{Context, Server};

const PERMISSION_PING: &str = "PumpkinPing:command.ping";
const PERMISSION_PING_OTHER: &str = "PumpkinPing:command.ping.other";
const PLAYER_ARGUMENT: &str = "player";

struct PingCommandExecutor;
impl CommandHandler for PingCommandExecutor {
    fn handle(&self, sender: CommandSender, _server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Players(players) = args.get_value(PLAYER_ARGUMENT) {
            for player in players {
                let ping = player.get_ping();

                let msg = player.get_display_name();
                msg.color_named(NamedColor::Green);

                let msg_second_part = TextComponent::text(" has a ping of ");
                msg_second_part.color_named(NamedColor::Gray);
                msg.add_child(msg_second_part);

                msg.add_text(format!("§{}{}ms", get_color(ping), ping).as_str());

                sender.send_message(msg);
            }

            return Ok(1);
        }

        match sender.as_player() {
            Some(player) => {
                let ping = player.get_ping();
                let color = get_color(ping);

                let msg = TextComponent::text(&format!("§7Your ping is §{}{}ms", color, ping));
                sender.send_message(msg);
            }
            None => {
                sender.send_message(TextComponent::text("You are not a player!"))
            }
        };

        Ok(1)
    }
}

fn get_color(ping: u32) -> &'static str {
    if ping < 30 {
        "a"
    } else if ping < 80 {
        "e"
    } else {
        "c"
    }
}

pub fn register_command(context: Context) -> pumpkin_plugin_api::Result<()> {

    context.register_permission(&Permission {
        node: PERMISSION_PING.to_string(),
        description: "Allows to show own ping".to_string(),
        default: PermissionDefault::Allow,
        children: Vec::new(),
    })?;

    context.register_command(init_command_tree(), PERMISSION_PING);

    Ok(())
}

fn init_command_tree() -> Command {
    let names = ["ping".to_string()];
    let description = "Show the ping of a player";

    let command = Command::new(&names, description);

    command.then(
        CommandNode::argument(PLAYER_ARGUMENT, &ArgumentType::Players).execute(PingCommandExecutor)
    );

    command.execute(PingCommandExecutor)
}
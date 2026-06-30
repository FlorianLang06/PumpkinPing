use pumpkin_plugin_api::command::{Command, CommandError, CommandNode, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::command_wit::{Arg, ArgumentType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::permission::{Permission, PermissionDefault, PermissionLevel};
use pumpkin_plugin_api::text::{NamedColor, TextComponent};
use pumpkin_plugin_api::{Context, Server};

pub const PERMISSION_PING: &str = "PumpkinPing:command.ping";
pub const PERMISSION_PING_OTHER: &str = "PumpkinPing:command.ping.other";
const PLAYER_ARGUMENT: &str = "player";

struct PingCommandExecutor;
impl CommandHandler for PingCommandExecutor {
    fn handle(&self, sender: CommandSender, server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        if let Arg::Players(players) = args.get_value(PLAYER_ARGUMENT) {
            if !sender.has_permission(&server, PERMISSION_PING_OTHER) {
                return Err(CommandError::PermissionDenied)
            }

            for player in players {
                let ping = player.get_ping();

                let msg = player.get_display_name();
                msg.color_named(NamedColor::Green);

                let msg_second_part = TextComponent::text(" has a ping of ");
                msg_second_part.color_named(NamedColor::Gray);
                msg.add_child(msg_second_part);

                msg.add_child(get_ping_msg_part(ping));

                sender.send_message(msg);
            }

            return Ok(1);
        }

        match sender.as_player() {
            Some(player) => {
                let ping = player.get_ping();

                let msg = TextComponent::text("Your ping is ");
                msg.color_named(NamedColor::Gray);

                msg.add_child(get_ping_msg_part(ping));

                sender.send_message(msg);
            }
            None => {
                let msg = TextComponent::text("You are not a player!");
                msg.color_named(NamedColor::Red);

                if sender.has_permission(&server, PERMISSION_PING_OTHER) {
                    let help_message = TextComponent::text(" You can use /ping <Playername> to see the ping of a player.");
                    help_message.color_named(NamedColor::Red);
                    msg.add_child(help_message);
                }

                sender.send_message(msg);
            }
        };

        Ok(1)
    }
}

fn get_ping_msg_part(ping: u32) -> TextComponent {
    let ping_part = TextComponent::text(format!("{}ms", ping).as_str());
    ping_part.color_named(get_color(ping));
    ping_part
}

fn get_color(ping: u32) -> NamedColor {
    if ping < 30 {
        NamedColor::Green
    } else if ping < 80 {
        NamedColor::Yellow
    } else {
        NamedColor::Red
    }
}

pub fn register_command(context: Context) -> pumpkin_plugin_api::Result<()> {
    context.register_permission(&Permission {
        node: PERMISSION_PING_OTHER.to_string(),
        description: "Allows to show ping of other players".to_string(),
        default: PermissionDefault::Op(PermissionLevel::Four),
        children: Vec::new(),
    })?;

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
use pumpkin_plugin_api::common::RgbColor;
use pumpkin_plugin_api::enchantments_wit::TextComponent;
use pumpkin_plugin_api::text::NamedColor;

pub fn prefix() -> TextComponent {
    TextComponent::text("[").color_named(NamedColor::Gray)
        .add_child(TextComponent::text("Pumpkin").color_rgb(RgbColor {r: 248, g: 180, b: 57}))
        .add_child(TextComponent::text("Ping").color_rgb(RgbColor {r: 247, g: 128, b: 19}))
        .add_child(TextComponent::text("] ").color_named(NamedColor::Gray))
}
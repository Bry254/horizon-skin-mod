// The commands this mod owns, so the chat box colours, completes and lists
// `/skin` and `/hskin` like the game's own commands.
fn horizon_skin_commands(commands: &mut Vec<&'static str>) {
    commands.push("skin");
    commands.push("hskin");
}

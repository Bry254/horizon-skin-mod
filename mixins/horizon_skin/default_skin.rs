// Every skin the game could not resolve (single player's local player among
// them) falls back to DefaultPlayerSkin.get. Answer with the mod's skin for
// the local player only, so the model wears it wherever it is resolved: the
// world, the first-person arm, the survival inventory and the creative one.

fn horizon_skin_default_skin(uuid: u128) -> std::ops::ControlFlow<&'static str> {
    match crate::tab_list::horizon_skin_hub::local_skin(uuid) {
        Some(location) => std::ops::ControlFlow::Break(location),
        None => std::ops::ControlFlow::Continue(()),
    }
}

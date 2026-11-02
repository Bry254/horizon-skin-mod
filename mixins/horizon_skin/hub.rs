// Shared with the game's frame hook: which UUID is the local player and the
// atlas sprite the mod stored its skin under. Kept here, in the renderer,
// because `default_skin` lives in this crate and the hook in the game crate.

use std::cell::Cell;

thread_local! {
    /// The local player's UUID and the skin location to answer with.
    static LOCAL: Cell<Option<(u128, &'static str)>> = const { Cell::new(None) };
}

/// The mod is on: answer `uuid` (when it is the local player) with `location`.
pub fn set_local(uuid: u128, location: &'static str) {
    LOCAL.with(|local| local.set(Some((uuid, location))));
}

/// The mod is off: give every UUID the game's own default skin back.
pub fn clear_local() {
    LOCAL.with(|local| local.set(None));
}

/// `location` for the local player, `None` for anyone else.
pub fn local_skin(uuid: u128) -> Option<&'static str> {
    LOCAL.with(|local| match local.get() {
        Some((own, location)) if own == uuid => Some(location),
        _ => None,
    })
}

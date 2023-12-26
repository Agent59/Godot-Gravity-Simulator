use godot::prelude::*;

struct GodotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GodotExtension {}

pub mod godot_aliases;
pub mod space;

pub mod base;

pub mod direct;
pub mod barnes_hut;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}

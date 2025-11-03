use serde::*;

/// Primary structure detailing a song.
#[derive(Deserialize, Serialize)]
pub struct Song {
    /// The metadata of the song.
    pub metadata: SongMetadata,
    /// The sets that are active.
    pub sets: u8,
    /// A list of all valid layers.
    pub layers: Vec<Layer>,
}

/// Defines generic song metadata.
///
/// Song metadata is used for displaying a song's information. This structure
/// defines this metadata.
#[derive(Deserialize, Serialize)]
pub struct SongMetadata {
    /// The name of the song internally.
    pub name: String,
    /// The display name of the song. Used for menus and similar.
    pub display_name: String,
    /// The authors of the song itself. This is typically the composers, lyricists, and vocalists.
    pub authors: Vec<String>,
    /// The charters of the song to display in menus.
    pub charters: Vec<String>,
}

/// A container for various hooks.
#[derive(Deserialize, Serialize)]
pub struct Layer(Vec<Hook>);

/// An enum defining what a hook can be constructed from.
#[derive(Deserialize, Serialize)]
pub enum HookComponent {
    /// A basic note.
    ///
    /// Notes are defined by their strumline and the actual strum. A note is
    /// done in this manner to allow better categorization.
    NOTE(u8, u8),
}

/// A hook that spawns an entity at the following location.
#[derive(Deserialize, Serialize)]
pub struct Hook(Vec<HookComponent>, Vec<u16>);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_basic_song() {
        let song: Song = Song {
            metadata: SongMetadata {
                name: String::from("basic-song"),
                display_name: String::from("Basic Song"),
                authors: vec![String::from("author")],
                charters: vec![String::from("charter")],
            },
            sets: 0b11111111,
            layers: vec![Layer(vec![Hook(vec![HookComponent::NOTE(0, 0)], vec![0])])],
        };

        println!(
            "{}",
            ron::ser::to_string_pretty(&song, ron::ser::PrettyConfig::default()).unwrap()
        );
    }
}

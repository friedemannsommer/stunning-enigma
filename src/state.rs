#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    StartUp,
    MainMenu,
    Loading,
    InGame,
}

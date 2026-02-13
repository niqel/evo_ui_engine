#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventKind {
    Tick,
    Exit,
    WindowResized,
    KeyPressed,
    KeyReleased,
    TextInput,
    MouseMoved,
    MouseClicked,
    MouseDown,
    MouseUp,
}

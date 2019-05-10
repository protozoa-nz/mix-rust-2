#[derive(Debug)]
pub enum DialState {
    OnHook,
    Dialing(String), 
    Connected(String),
    Disconnecting(String)
}

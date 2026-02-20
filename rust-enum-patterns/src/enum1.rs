//Pattern 1: Representing States
//When something has distinct modes/states:
#[derive(Debug)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Failed(String),    // failed carries a reason
}

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionRefused,
    QueryFailed(String),
    Timeout { after_secs: u64 },
    NotFound { table: String, id: u64 },
}
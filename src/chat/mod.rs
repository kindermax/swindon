mod cid;
mod authorize;
mod backend;
mod message;
mod tangle_auth;
mod processor;
mod error;
mod close_reason;
mod listener;
mod dispatcher;
mod connection_sender;
mod inactivity_handler;
mod replication;

pub use self::cid::Cid;
pub use self::tangle_auth::TangleAuth;
pub use self::authorize::start_authorize;
pub use self::message::{Meta, Args, Kwargs};
pub use self::error::MessageError;
pub use self::close_reason::CloseReason;
pub use self::listener::SessionPools;
pub use self::processor::{Processor, ConnectionMessage};
pub use self::dispatcher::Dispatcher;
pub use self::connection_sender::ConnectionSender;
pub use self::replication::ReplicationSession;

use metrics::{Counter, Integer, List, Metric};

lazy_static! {
    pub static ref CONNECTS: Counter = Counter::new();
    pub static ref CONNECTIONS: Integer = Integer::new();
    pub static ref FRAMES_SENT: Counter = Counter::new();
}

pub struct Shutdown;

pub fn metrics() -> List {
    vec![
        (Metric("websockets.swindon_chat", "connects"), &*CONNECTS),
        (Metric("websockets.swindon_chat", "connections"), &*CONNECTIONS),
        (Metric("websockets.swindon_chat", "frames_received"),
            &*dispatcher::FRAMES_RECEIVED),
        (Metric("websockets.swindon_chat", "frames_sent"), &*FRAMES_SENT),
        (Metric("websockets.swindon_chat", "session_pools"),
            &*processor::SESSION_POOLS),
        (Metric("websockets.swindon_chat", "active_sessions"),
            &*processor::ACTIVE_SESSIONS),
        (Metric("websockets.swindon_chat", "inactive_sessions"),
            &*processor::INACTIVE_SESSIONS),
        (Metric("websockets.swindon_chat.pubsub", "input_messages"),
            &*processor::PUBSUB_INPUT),
        (Metric("websockets.swindon_chat.pubsub", "output_messages"),
            &*processor::PUBSUB_OUTPUT),
        (Metric("websockets.swindon_chat.pubsub", "topics"),
            &*processor::TOPICS),
        (Metric("websockets.swindon_chat.lattice", "namespaces"),
            &*processor::LATTICES),
        (Metric("websockets.swindon_chat.lattice.shared", "keys"),
            &*processor::SHARED_KEYS),
        (Metric("websockets.swindon_chat.lattice.shared", "counters"),
            &*processor::SHARED_COUNTERS),
        (Metric("websockets.swindon_chat.lattice.shared", "sets"),
            &*processor::SHARED_SETS),
        (Metric("websockets.swindon_chat.lattice.private", "keys"),
            &*processor::PRIVATE_KEYS),
        (Metric("websockets.swindon_chat.lattice.private", "counters"),
            &*processor::PRIVATE_COUNTERS),
        (Metric("websockets.swindon_chat.lattice.private", "sets"),
            &*processor::PRIVATE_SETS),
        (Metric("websockets.swindon_chat.lattice", "set_items"),
            &*processor::SET_ITEMS),
        (Metric("replication", "connections"),
            &*replication::CONNECTIONS),
        (Metric("replication", "frames_sent"),
            &*replication::FRAMES_SENT),
        (Metric("replication", "frames_received"),
            &*replication::FRAMES_RECEIVED),
    ]
}

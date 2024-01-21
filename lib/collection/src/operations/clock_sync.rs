use crate::shards::shard::PeerId;

#[derive(Debug, Clone, Default)]
pub struct ClockSync {
    pub tick: u64,
    pub peer_id: PeerId,
    pub thread_id: u64,
}

impl ClockSync {
    pub fn new(tick: u64, peer_id: PeerId, thread_id: u64) -> Self {
        Self {
            tick,
            peer_id,
            thread_id,
        }
    }
}

impl From<ClockSync> for api::grpc::qdrant::ClockSync {
    fn from(clock_sync: ClockSync) -> Self {
        Self {
            tick: clock_sync.tick,
            peer_id: clock_sync.peer_id,
            thread_id: clock_sync.thread_id,
        }
    }
}

impl From<api::grpc::qdrant::ClockSync> for ClockSync {
    fn from(clock_sync: api::grpc::qdrant::ClockSync) -> Self {
        Self {
            tick: clock_sync.tick,
            peer_id: clock_sync.peer_id,
            thread_id: clock_sync.thread_id,
        }
    }
}

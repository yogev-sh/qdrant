use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use crate::operations::clock_sync::ClockSync;
use crate::shards::shard::PeerId;

pub struct VectorClock {
    counters: Vec<Arc<AtomicU64>>,
    availabilities: Vec<Arc<AtomicBool>>,
}

pub struct ClockGuard {
    id: usize,
    counter: Arc<AtomicU64>,
    availability: Arc<AtomicBool>,
}

impl ClockGuard {
    pub fn new(id: usize, counter: Arc<AtomicU64>, availability: Arc<AtomicBool>) -> Self {
        Self {
            id,
            counter,
            availability,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn resolve(self) {
        self.availability.swap(false, Ordering::Relaxed);
    }

    pub fn get_clock_sync(&self, peer_id: PeerId) -> ClockSync {
        ClockSync {
            tick: self.counter.load(Ordering::Relaxed),
            peer_id,
            thread_id: self.id as u64,
        }
    }

    pub fn increment(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }

    fn release(&mut self) {
        self.availability.swap(false, Ordering::Relaxed);
    }
}

impl Drop for ClockGuard {
    fn drop(&mut self) {
        self.release();
    }
}

impl VectorClock {
    pub fn new() -> Self {
        Self {
            counters: Vec::new(),
            availabilities: Vec::new(),
        }
    }

    pub fn occupy(&mut self) -> ClockGuard {
        for (id, availability) in self.availabilities.iter().enumerate() {
            if availability.swap(false, Ordering::SeqCst) {
                self.counters[id].fetch_add(1, Ordering::SeqCst);
                return ClockGuard::new(id, self.counters[id].clone(), availability.clone());
            }
        }
        let new_availability = Arc::new(AtomicBool::new(false));
        let new_counter_value = 1;

        let new_counter = Arc::new(AtomicU64::new(new_counter_value));
        self.availabilities.push(new_availability.clone());
        self.counters.push(new_counter.clone());
        ClockGuard::new(self.counters.len() - 1, new_counter, new_availability)
    }
}

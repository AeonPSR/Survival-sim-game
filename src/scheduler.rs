use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use crate::types::{EntityId, EventId};

// ============================================================================
// EVENT TYPES
// ============================================================================

/// All possible events that can occur in the simulation.
/// Each variant can carry data (like which entity it affects).
/// Add new variants here as the simulation grows.
#[derive(Debug, Clone)]
pub enum EventType {
    HungerCritical { entity: EntityId },
    DiseaseRoll { entity: EntityId },
}

// ============================================================================
// EVENT
// ============================================================================

/// A scheduled event: what happens, when, and a unique ID for cancellation.
#[derive(Debug, Clone)]
pub struct Event {
    pub id: EventId,
    pub cycle: u64,
    pub event_type: EventType,
}

// ----------------------------------------------------------------------------
// Trait implementations for Event
// These tell Rust how to compare Events so BinaryHeap can sort them.
// ----------------------------------------------------------------------------

/// Marker trait: Events can be compared for equality
impl Eq for Event {}

/// How to check if two events are "equal" (same cycle)
/// Events are composed of several stuff, Rust just can't guess which stuff
/// It should use to compare things !!!!
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.cycle == other.cycle
    }
}

/// How to order events.
/// IMPORTANT: We reverse the comparison (other vs self) so that
/// LOWER cycles have HIGHER priority. This makes BinaryHeap act
/// as a min-heap (soonest event first) instead of max-heap.
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        // Swap other/self to reverse the order
        other.cycle.cmp(&self.cycle)
    }
}

/// Required by Ord - just delegates to cmp()
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// SCHEDULER
// ============================================================================

/// The event scheduler - heart of the event-driven simulation.
/// 
/// Instead of checking every entity every cycle (polling), we only
/// process events when they're scheduled to happen.
pub struct Scheduler {
    /// Priority queue of pending events, sorted by cycle (soonest first)
    events: BinaryHeap<Event>,
    
    /// Set of event IDs that have been cancelled.
    /// We don't remove from the heap (expensive), we just mark as cancelled
    /// and skip them when they come up.
    cancelled: HashSet<EventId>,
    
    /// Counter to generate unique event IDs
    next_event_id: EventId,
}

impl Scheduler {
    /// Create a new empty scheduler
    pub fn new() -> Self {
        Scheduler {
            events: BinaryHeap::new(),
            cancelled: HashSet::new(),
            next_event_id: 0,
        }
    }
    
    /// Schedule an event to fire at a specific cycle.
    /// Returns the event ID so you can cancel it later if needed.
    /// 
    /// Example:
    ///   let id = scheduler.schedule(100, EventType::HungerCritical { entity: 0 });
    ///   // Later, if entity eats:
    ///   scheduler.cancel(id);
    ///   scheduler.schedule(200, EventType::HungerCritical { entity: 0 });
    pub fn schedule(&mut self, cycle: u64, event_type: EventType) -> EventId {
        let id = self.next_event_id;
        self.next_event_id += 1;
        
        self.events.push(Event {
            id,
            cycle,
            event_type,
        });
        
        id
    }
    
    /// Cancel a previously scheduled event by its ID.
    /// The event stays in the heap but will be skipped when popped.
    pub fn cancel(&mut self, id: EventId) {
        self.cancelled.insert(id);
    }
    
    /// Get the next event that's due (cycle <= current_cycle).
    /// Returns None if no events are due.
    /// 
    /// Call this in a loop to process all events for the current cycle:
    ///   while let Some(event) = scheduler.pop_due(current_cycle) {
    ///       // handle event
    ///   }
    pub fn pop_due(&mut self, current_cycle: u64) -> Option<Event> {
        while let Some(event) = self.events.peek() {
            // Skip cancelled events (lazy deletion)
            if self.cancelled.contains(&event.id) {
                self.events.pop();
                continue;
            }
            // Check if this event is due
            if event.cycle <= current_cycle {
                let event = self.events.pop().unwrap();
                return Some(event);
            }
            // Event is in the future, stop looking
            break;
        }
        None
    }
}
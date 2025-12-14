use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use crate::types::{EntityId, EventId};

#[derive(Debug, Clone)]
pub enum EventType {
    HungerCritical { entity: EntityId },
    DiseaseRoll { entity: EntityId },
}

#[derive(Debug, Clone)]
pub struct Event {
    pub id: EventId,
    pub cycle: u64,
    pub event_type: EventType,
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.cycle == other.cycle
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cycle.cmp(&self.cycle)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Scheduler {
    events: BinaryHeap<Event>,
    cancelled: HashSet<EventId>,
    next_event_id: EventId,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            events: BinaryHeap::new(),
            cancelled: HashSet::new(),
            next_event_id: 0,
        }
    }
    
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
    
    pub fn cancel(&mut self, id: EventId) {
        self.cancelled.insert(id);
    }
    
    pub fn pop_due(&mut self, current_cycle: u64) -> Option<Event> {
        while let Some(event) = self.events.peek() {
            if self.cancelled.contains(&event.id) {
                self.events.pop();
                continue;
            }
            if event.cycle <= current_cycle {
                let event = self.events.pop().unwrap();
                return Some(event);
            }
            break;
        }
        None
    }
}
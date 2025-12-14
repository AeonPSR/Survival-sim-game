use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;

type EntityId = u32;
type EventId = u32;

#[derive(Debug, Clone)]
enum EventType {
	HungerCritical { entity: EntityId },
	DiseaseRoll { entity: EntityId },
	// Add more as needed
}

#[derive(Debug, Clone)]
struct Event {
	id: EventId,
	cycle: u64,
	event_type: EventType,
}

// Make Event comparable (for the priority queue)
impl Eq for Event {}

impl PartialEq for Event {
	fn eq(&self, other: &Self) -> bool {
		self.cycle == other.cycle
	}
}

impl Ord for Event {
	fn cmp(&self, other: &Self) -> Ordering {
		// Reverse order! Lower cycle = higher priority
		other.cycle.cmp(&self.cycle)
	}
}

impl PartialOrd for Event {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

struct Scheduler {
	events: BinaryHeap<Event>,
	cancelled: HashSet<EventId>,
	next_event_id: EventId,
}

impl Scheduler {
	fn new() -> Self {
		Scheduler {
			events: BinaryHeap::new(),
			cancelled: HashSet::new(),
			next_event_id: 0,
		}
	}
	
	fn schedule(&mut self, cycle: u64, event_type: EventType) -> EventId {
		let id = self.next_event_id;
		self.next_event_id += 1;
		
		self.events.push(Event {
			id,
			cycle,
			event_type,
		});
		
		id  // Return the ID so caller can cancel it later
	}
	
	fn cancel(&mut self, id: EventId) {
		self.cancelled.insert(id);
	}
	
	fn pop_due(&mut self, current_cycle: u64) -> Option<Event> {
		// Skip cancelled events
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

struct Hunger {
	current: f32,
	decay_rate: f32,
}

fn hunger_system(hungers: &mut HashMap<EntityId, Hunger>) {
	for (_entity_id, hunger) in hungers {
		hunger.current -= hunger.decay_rate;
		
		if hunger.current < 0.0 {
			hunger.current = 0.0;
		}
	}
}

fn main() {
    let mut hungers: HashMap<EntityId, Hunger> = HashMap::new();
    let mut scheduler = Scheduler::new();
    
    // Create entities with hunger
    hungers.insert(0, Hunger { current: 100.0, decay_rate: 5.0 });
    hungers.insert(1, Hunger { current: 80.0, decay_rate: 3.0 });
    
    // Schedule initial "hunger critical" events
    // Entity 0: 100 → 5 = 95 points to drop, at 5/cycle = 19 cycles
    scheduler.schedule(19, EventType::HungerCritical { entity: 0 });
    // Entity 1: 80 → 5 = 75 points to drop, at 3/cycle = 25 cycles
    scheduler.schedule(25, EventType::HungerCritical { entity: 1 });
    
    // Simulation loop
    let max_cycles = 30;
    
    for current_cycle in 0..max_cycles {
        // Process all events due this cycle
        while let Some(event) = scheduler.pop_due(current_cycle) {
            match event.event_type {
                EventType::HungerCritical { entity } => {
                    println!("Cycle {}: Entity {} is critically hungry!", current_cycle, entity);
                    // Could schedule disease rolls here
                    scheduler.schedule(current_cycle + 1, EventType::DiseaseRoll { entity });
                }
                EventType::DiseaseRoll { entity } => {
                    println!("Cycle {}: Entity {} disease check (would roll dice)", current_cycle, entity);
                    // Reschedule if still critical (for now, just run 3 times)
                    if current_cycle < 28 {
                        scheduler.schedule(current_cycle + 1, EventType::DiseaseRoll { entity });
                    }
                }
            }
        }
    }
    
    println!("Simulation complete!");
}
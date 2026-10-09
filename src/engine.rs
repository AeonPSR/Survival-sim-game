use std::collections::BTreeMap;
use crate::Event;

pub type Time = u64; //Internal unit of time

// EventID are made with the combo of their time and global sequencer value.
pub type EventID = (Time, u64);

#[derive(PartialEq)]
enum State {Future, Done, Canceled}

#[derive(PartialEq)]
struct Scheduled {
	state: State,
	event: crate::Event,
}

pub struct Scheduler {
	timeline: BTreeMap<EventID, Scheduled>,
	current_time: Time,
	current_seq: u64,
}

//Okay so it's super trippy to have another thing called "Scheduler" here, but it's normal.
//"impl" is for "implementation" or something.
//So "Hey I'm IMPLementing this shit for the 'Scheduler' thing I created."
impl Scheduler {
	//Initializer, so we put the actual starting values in it.
	pub fn new() -> Scheduler { 
		Scheduler {
			timeline: BTreeMap::new(),
			current_time: 0,
			current_seq: 0,
		}
	}
	
	pub fn now(&self) -> Time {
		self.current_time
	}
	
	//"&mut self" isn't going to be put has an argument between (), but has a dot. Like "scheduler.schedule_at(time, event)"
	pub fn schedule_at(&mut self, time: Time, event: Event) -> Option<EventID> {
		//Check for trying to plan in the past
		if time < self.current_time {
			eprintln!("ERROR: Attempted scheduling in the past. Current time: {}. Attempted at {}. [schedule_at]", self.current_time, time);
			return None;
		}
		
		//Once checks are done, we insert it on the requested time slot, increasing the eq value.
		self.current_seq += 1;
		let event_id = (time, self.current_seq);
		self.timeline.insert(event_id, Scheduled {
			state: State::Future,
			event,
		});
		
		return Some(event_id);
	}
	
	pub fn schedule_in(&mut self, delay: Time, event: Event) -> Option<EventID> {
		return self.schedule_at(self.current_time + delay, event)
	}
	
	pub fn cancel(&mut self, id: EventID) {
		if let Some(thing) = self.timeline.get_mut(&id) {
			if thing.state == State::Future {
				thing.state = State::Canceled;
				return ;
			}
			eprintln!("ERROR: Scheduled ID {:?} already Done or Canceled. [cancel()]", id);
			return ;
		}
		eprintln!("ERROR: Scheduled ID {:?} not found. [cancel()]", id);
	}
	
	//Main iteration
	pub fn pop_next(&mut self) -> Option<(Time, Event)> {
		let mut found: Option<EventID> = None;
		//We find the next event in our current time
		for (event_id, scheduled) in self.timeline.range((self.current_time, 0)..) { 
			if scheduled.state == State::Future {
				found = Some(*event_id); 
				break ;
			}
		}
		let present_id = found?; //? -> If "found" is empty, exit the function
		let present = self.timeline.get_mut(&present_id).unwrap();
		present.state = State::Done;
		let present_clone = present.event.clone();
		//We update the time of the Scheduler to the time of the latest event.
		self.current_time = present_id.0; 
		return Some((present_id.0, present_clone))
	}
}

//Wait no, we should need something else (The main ?) to call on this function in a loop and use the OUTPUT of this function to then run the associated code. This would prevent any timing issue, while better encapsulating stuff.
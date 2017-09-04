use std::ops::{Generator, GeneratorState};
use std::collections::{HashSet, HashMap};
use std::cell::RefCell;

use yield_instruction::YieldInstruction;

/// Unique identifier for a coroutine. After starting a corutine, use this to refer to that coroutine
pub struct CoroutineHandle(usize);

/// Conditions determining when to resume a coroutine. There are some pre-defined options, and a hook for defining a trait object with arbitrary logic
pub enum CoroutineCondition {
	WaitOneFrame,
	WaitForCoroutine(CoroutineHandle),
	YieldInstruction(Box<YieldInstruction>),
}

struct Coroutine {
	condition: CoroutineCondition,
	generator: Box<Generator<Yield=CoroutineCondition, Return=()>>,
}

struct CoroutineQueue {
	queued_coroutines: HashMap<usize, Box<Generator<Yield=CoroutineCondition, Return=()>>>,
	next_id: usize,
}
impl CoroutineQueue {
	fn new() -> Self {
		Self {
			queued_coroutines: HashMap::new(),
			next_id: 1,
		}
	}
	fn get_next_id(&mut self) -> usize {
		let next_id = self.next_id;
		self.next_id += 1;
		next_id
	}
}

struct CurrentCoroutines {
	current_coroutine_ids: HashSet<usize>,
	current_coroutines: HashMap<usize, Coroutine>,
}
impl CurrentCoroutines {
	fn new() -> Self {
		Self {
			current_coroutine_ids: HashSet::new(),
			current_coroutines: HashMap::new(),
		}
	}
	fn update(&mut self) {
		//for each coroutine, check if we should resume the coroutine, then check the yield vlaue to see if it's ended
		let mut finished_coroutines: Vec<usize> = Vec::new();
		for (id, coroutine) in self.current_coroutines.iter_mut() {
			let should_resume = match coroutine.condition {
				CoroutineCondition::WaitOneFrame => true,
				CoroutineCondition::WaitForCoroutine(CoroutineHandle(id)) => !self.current_coroutine_ids.contains(&id),
				CoroutineCondition::YieldInstruction(ref instruction) => instruction.should_resume(),
			};

			if should_resume {
				let resume_result = coroutine.generator.resume();

				match resume_result {
					GeneratorState::Yielded(condition) => {
						coroutine.condition = condition;
					}
					GeneratorState::Complete(()) => {
						finished_coroutines.push(*id)
					}
				}
			}
		}

		for id in finished_coroutines {
			self.current_coroutine_ids.remove(&id);
			self.current_coroutines.remove(&id);
		}
	}
}

/// CorputineManager manages the state and execution of several coroutines. Queue a new coroutine by calling start_coroutine(), and update all coroutines by calling update() once per frame
pub struct CoroutineManager {
	queue: RefCell<CoroutineQueue>,
	coroutines: RefCell<CurrentCoroutines>,
}
impl CoroutineManager {
	pub fn new() -> CoroutineManager {
		Self {
			queue: RefCell::new(CoroutineQueue::new()),
			coroutines: RefCell::new(CurrentCoroutines::new()),
		}
	}
	pub fn start_coroutine<G>(&self, coroutine: G) -> CoroutineHandle where G: Generator<Yield=CoroutineCondition, Return=()> + 'static {
		let mut queue_ref = self.queue.borrow_mut();

		let id = queue_ref.get_next_id();
		queue_ref.queued_coroutines.insert(id, Box::new(coroutine));

		CoroutineHandle(id)
	}
	pub fn update(&self) {
		let mut coroutine_ref = self.coroutines.borrow_mut();

		{
			//drain the queue of new coroutines into the list of active corutines
			//it's important to do this inside of an isolated scape, because coroutines may want to start new coroutines from within themselves
			//so the borrow on the queue must be released before we start updating the coroutines
			let mut queue_ref = self.queue.borrow_mut();
			for (k, generator) in queue_ref.queued_coroutines.drain() {
				coroutine_ref.current_coroutine_ids.insert(k);
				coroutine_ref.current_coroutines.insert(k, Coroutine { condition: CoroutineCondition::WaitOneFrame, generator});
			}
		}

		coroutine_ref.update();
	}
	pub fn has_coroutines(&self) -> bool {
		self.coroutines.borrow().current_coroutines.len() > 0 || self.queue.borrow().queued_coroutines.len() > 0
	}
}
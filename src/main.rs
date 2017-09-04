#![feature(generators, generator_trait)]

use std::cell::Cell;
use std::rc::Rc;

mod yield_instruction;
mod coroutine;

use yield_instruction::WaitUntil;

use coroutine::{CoroutineManager, CoroutineCondition};

fn main() {
	let call_count_a: Rc<Cell<usize>> = Rc::new(Cell::new(0));
	let call_count_b = call_count_a.clone();

	let manager = Rc::new(CoroutineManager::new());
	let manager_ref = manager.clone();

	let handle1 = manager.start_coroutine(move || {
		for _ in 0..20 {
			let prev_count = call_count_a.get();
			call_count_a.set(prev_count + 1);

			println!("coroutine1: {}", prev_count + 1);

			yield CoroutineCondition::WaitOneFrame;
		}
	});

	let handle2 = manager.start_coroutine(move || {
		let target_count = 15;
		println!("coroutine2: Waiting for call count to be {}", target_count);

		yield CoroutineCondition::YieldInstruction(
			WaitUntil::new(move|| {
				call_count_b.get() == target_count
			})
		);
		println!("coroutine2: Call count has reached {}", target_count);

		let handle4 = manager_ref.start_coroutine(move || {
			println!("coroutine4: started");
			for i in 0..10 {
				yield CoroutineCondition::WaitOneFrame;
				println!("coroutine4: {}", i * 10);
			}
		});

		yield CoroutineCondition::WaitForCoroutine(handle4);

		println!("coroutine2: coroutine 4 has finished");

		for i in 0..10 {
			yield CoroutineCondition::WaitOneFrame;
			println!("coroutine2: {}", i);
		}
	});

	manager.start_coroutine(move || {
		println!("coroutine3: Waiting for coroutine1 to finish");

		yield CoroutineCondition::WaitForCoroutine(handle1);

		println!("coroutine3: coroutine1 has finished, waiting for coroutine2");

		yield CoroutineCondition::WaitForCoroutine(handle2);

		println!("coroutine3: coroutine2 has finished");

		for i in 0..10 {
			yield CoroutineCondition::WaitOneFrame;
			println!("coroutine3 {}", i);
		}
	});

	while manager.has_coroutines() {
		manager.update();
	}
}
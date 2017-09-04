
/// Mirror of unity's CustomYieldInstruction. should_resume is called each frame. If it returns true, resume() will be called on the coroutine
pub trait YieldInstruction {

	/// Called each frame. Return true if resume() should be called on the generator this frame, return false to wait
	fn should_resume(&self) -> bool;
}


/// WaitUntil is a yield instruction that suspends the coroutine until the provided predicate returns true
pub struct WaitUntil<F> {
	predicate: F,
}
impl<F> WaitUntil<F> where F: Fn() -> bool {
	pub fn new(predicate: F) -> Box<Self> {
		Box::new(Self {
			predicate
		})
	}
}
impl<F> YieldInstruction for WaitUntil<F> where F: Fn() -> bool {
	fn should_resume(&self) -> bool {
		return (self.predicate)();
	}
}

/// WaitForSeconds is a yield instruction that (theoretically)suspends the coroutine until the specified number of seconds have passed
pub struct WaitForSeconds {
	finish_time: f32,
}
impl WaitForSeconds {
	pub fn new(seconds: f32) -> Box<Self> {
		//todo: we need a way to get the current game time
		let current_time = 100.0f32;

		Box::new(Self {
			finish_time: current_time + seconds
		})
	}
}
impl YieldInstruction for WaitForSeconds {
	fn should_resume(&self) -> bool {
		//todo: we need a way to get the current game time
		let current_time = 110.0f32;

		return current_time > self.finish_time;
	}
}
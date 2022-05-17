use std::time;

//
// Stopwatch that when it exits scope (is dropped) prints the elapsed time
// Use for one shot time keeping, when you don't need a record other than in stdout
//
pub struct ScopedStopwatch {
    start : Option<time::Instant>,
    id : String
}

impl ScopedStopwatch {
    pub fn new(id: String) -> Self {
        ScopedStopwatch { start: None, id }
    }

    pub fn begin(&mut self) {
        self.start = Some(time::Instant::now());
    }

    pub fn new_begin(id: String) -> Self {
        let mut s = Self::new(id);

        s.begin();

        s
    }
}

impl Drop for ScopedStopwatch {
    fn drop(&mut self) {
        println!("{} took {}s", self.id, (time::Instant::now() - self.start.unwrap()).as_secs_f32());
    }
}
pub struct Fs {}

impl Fs {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) {}
}

impl Default for Fs {
    fn default() -> Self {
        Self::new()
    }
}


pub struct RingBuffer {
    buffer: [u8; 10],
    index: usize,
    pub complete: bool
}

impl RingBuffer {
    pub fn new() -> RingBuffer {
        RingBuffer {
            buffer: [32; 10], // spaces
            index: 0,
            complete: false
        }
    }

    pub fn reset(&mut self) {
        for c in self.buffer.iter_mut() {
            *c = 32; // space
        }

        self.index = 0;
        self.complete = false;
    }

    pub fn set_complete(&mut self, value: bool) {
        self.complete = value;
    }

    pub fn is_complete(&self) -> bool {
        self.complete
    }

    pub fn add(&mut self, c: u8) {
        self.buffer[self.index] = c;
        self.index = (self.index + 1) % 10;
    }

    pub fn equals(&self, s: &str) -> bool {
        if s.len() > 10 {
            return false;
        }

        for (index, c) in s.chars().enumerate() {
            if u32::from(self.buffer[index]) != c.to_digit(10).unwrap() {
                return false;
            }
        }

        true
    }
}
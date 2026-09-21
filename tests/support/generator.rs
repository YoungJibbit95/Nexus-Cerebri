//! Test-only reproducible generator. No clock, entropy, or production dependency.
#![allow(dead_code)]
pub struct Generator(pub u64);
impl Generator {
    pub fn pick(&mut self, bound: u64) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        (z ^ (z >> 31)) % bound
    }
    pub fn shuffle<T>(&mut self, values: &mut [T]) {
        for i in (1..values.len()).rev() {
            let j = self.pick((i + 1) as u64) as usize;
            values.swap(i, j);
        }
    }
}

//! Kleiner, schneller Zufallsgenerator (xorshift64*) für die Aufgaben-Generatoren.

#[derive(Clone)]
pub struct Rng(u64);

impl Rng {
    pub fn neu() -> Self {
        let a = (js_sys::Math::random() * 4_294_967_296.0) as u64;
        let b = (js_sys::Math::random() * 4_294_967_296.0) as u64;
        Self::mit_seed((a << 32) ^ b)
    }

    pub const fn mit_seed(seed: u64) -> Self {
        Self(if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed })
    }

    pub const fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Zufallszahl im Bereich `lo..=hi`.
    pub fn zahl(&mut self, lo: i64, hi: i64) -> i64 {
        let span = (hi - lo + 1) as u64;
        lo + (self.next() % span) as i64
    }

    pub fn idx(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    pub fn bool(&mut self) -> bool {
        self.next() & 1 == 1
    }

    pub fn wahl<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        &v[self.idx(v.len())]
    }

    pub fn mischen<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.idx(i + 1);
            v.swap(i, j);
        }
    }
}

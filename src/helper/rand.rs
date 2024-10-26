use std::time::{SystemTime, UNIX_EPOCH};

fn get_random_index(range: usize) -> usize {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (now.as_nanos() as usize) % range
}

pub fn shuffle<T>(vec: &mut [T]) {
    let len = vec.len();
    for i in (1..len).rev() {
        let j = get_random_index(i + 1);
        vec.swap(i, j);
    }
}

pub trait Shuffle {
    fn shuffle(&mut self);
}
impl<T> Shuffle for [T] {
    fn shuffle(&mut self) {
        shuffle(self);
    }
}

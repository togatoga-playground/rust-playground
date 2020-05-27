use std::ops::{Deref, DerefMut, Index, IndexMut};

use pool::{Id, RegionAllocator};

#[derive(Clone, Copy)]
union DataWord {
    size: usize,
    x: i32,
}

struct Data<'a> {
    values: &'a mut [DataWord],
}

impl<'a> Index<usize> for Data<'a> {
    type Output = i32;
    fn index(&self, idx: usize) -> &Self::Output {
        unsafe { &self.values[idx].x }
    }
}

impl<'a> IndexMut<usize> for Data<'a> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        unsafe { &mut self.values[idx].x }
    }
}

type DataWordId = Id<DataWord>;

struct Pool {
    ra: RegionAllocator<DataWord>,
}

impl Pool {
    fn new() -> Pool {
        Pool {
            ra: RegionAllocator::default(),
        }
    }

    fn alloc(&mut self, values: &[i32]) -> DataWordId {
        let src = self.ra.alloc(DataWord { size: values.len() });
        values.iter().for_each(|x| {
            self.ra.alloc(DataWord { x: *x });
        });

        src
    }

    fn get_mut(&mut self, id: Id<DataWord>) -> Data {
        let src = self.ra.get(id);
        unsafe {
            let size = src.size;
            let values = self.ra.subslice_mut(id + 1, size);
            Data { values }
        }
    }
}

fn main() {
    let mut pool = Pool::new();
    let mut indices = vec![];
    for i in 1..10 {
        let values = (0..i).collect::<Vec<i32>>();
        eprintln!("{:?}", values);
        indices.push(pool.alloc(&values));
    }

    for id in indices {
        let data = pool.get_mut(id);
    }
}

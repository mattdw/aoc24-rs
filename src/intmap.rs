#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntMap<T> {
    pub cells: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Default + Clone> IntMap<T> {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> IntMap<T> {
        let cells = vec![T::default(); width * height];
        IntMap {
            cells,
            width,
            height,
        }
    }

    pub fn from_raw<F: Fn(char) -> T>(input: &str, mapper: F) -> IntMap<T> {
        let width = input
            .trim()
            .replace('\r', "")
            .find('\n')
            .expect("must be row-wrapped with newlines");
        let data: Vec<T> = input
            .trim()
            .replace(['\n', '\r', ' ', '\t'], "")
            .chars()
            .map(mapper)
            .collect();

        let height = data.len() / width;

        assert_eq!(data.len(), width * height);
        IntMap {
            cells: data,
            width,
            height,
        }
    }

    pub fn in_bounds(&self, co: (isize, isize)) -> bool {
        0 <= co.0 && co.0 < self.width as isize && 0 <= co.1 && co.1 < self.height as isize
    }

    pub fn pt_to_idx(&self, co: (isize, isize)) -> isize {
        co.1 * self.width as isize + co.0
    }

    pub fn idx_to_pt(&self, idx: isize) -> (isize, isize) {
        (idx % self.width as isize, idx / self.width as isize)
    }

    pub fn get(&self, co: (isize, isize)) -> Option<&T> {
        if !self.in_bounds(co) {
            return None;
        }
        self.cells.get(self.pt_to_idx(co) as usize)
    }

    #[allow(dead_code)]
    pub fn set(&mut self, co: (isize, isize), val: T) {
        let idx = self.pt_to_idx(co);
        self.cells[idx as usize] = val;
    }
}

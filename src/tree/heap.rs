#[derive(PartialEq, Debug)]
pub struct Heap {
    slice: Vec<i64>,
    is_max: bool,
}

impl Heap {
    pub fn new(slice: Vec<i64>, is_max: bool) -> Self {
        Heap { slice, is_max }.as_heap()
    }
    pub fn is_max(&self) -> bool {
        self.is_max
    }
    pub fn to_heap(mut self, is_max: bool) -> Self {
        self.is_max = is_max;
        self.as_heap()
    }
    fn as_heap(mut self) -> Self {
        for i in (1..=(Self::parent(self.size()))).rev() {
            self.shift_down_uncheck(i);
        }
        self
    }
    pub fn delete_node(mut self, i: usize) -> Self {
        if i >= self.size() {
            return self;
        }
        self.swap_uncheck(i + 1, self.size());
        self.slice.pop();
        self.shift_down_uncheck(i + 1);
        self
    }
    pub fn add_node(mut self, v: i64) -> Self {
        self.slice.push(v);
        self.shift_up_uncheck(self.size());
        self
    }
    /// O(nlog(n))
    pub fn sort(mut self) -> Vec<i64> {
        let raw_size = self.size();
        while self.size() > 0 {
            self.swap_uncheck(1, self.size());
            unsafe { self.slice.set_len(self.size() - 1) };
            self = self.as_heap();
        }
        unsafe { self.slice.set_len(raw_size) }
        if !self.is_max {
            self.slice.reverse()
        }
        return self.slice;
    }
    pub fn find_min_k(mut self, k: usize) -> Vec<i64> {
        if k >= self.size() {
            return self.slice;
        }
        let right: Vec<i64> = Vec::from(&self.slice[k..]);
        unsafe { self.slice.set_len(k) };
        self = self.to_heap(true);
        for v in right.iter() {
            if &self.slice[0] <= v {
                continue;
            }
            self.slice[0] = v.clone();
            self.shift_down_uncheck(1);
        }
        self.slice
    }
    fn parent(i: usize) -> usize {
        i / 2
    }
    fn left(i: usize) -> usize {
        i * 2
    }
    fn right(i: usize) -> usize {
        i * 2 + 1
    }
    fn size(&self) -> usize {
        self.slice.len()
    }
    fn get_uncheck(&self, i: usize) -> &i64 {
        self.slice.get(i - 1).unwrap_or(&0)
    }
    fn swap_uncheck(&mut self, i: usize, j: usize) {
        self.slice.swap(i - 1, j - 1)
    }
    fn elder(&self, a: &i64, b: &i64) -> bool {
        let c = a - b;
        if c > 0 {
            return self.is_max;
        }
        if c < 0 {
            return !self.is_max;
        }
        return false;
    }
    /// O(log(n)
    fn shift_down_uncheck(&mut self, i: usize) {
        let l = Self::left(i);
        let r = Self::right(i);
        let mut oldest = i;
        if l <= self.size() && self.elder(self.get_uncheck(l), self.get_uncheck(i)) {
            oldest = l;
        }
        if r <= self.size() && self.elder(self.get_uncheck(r), self.get_uncheck(oldest)) {
            oldest = r;
        }
        if oldest != i {
            self.swap_uncheck(oldest, i);
            self.shift_down_uncheck(oldest);
        }
    }
    /// O(log(n)
    fn shift_up_uncheck(&mut self, i: usize) {
        if i == 0 || i == 1 {
            return;
        }
        let p = Self::parent(i);
        if self.elder(self.get_uncheck(i), self.get_uncheck(p)) {
            self.swap_uncheck(i, p);
            self.shift_up_uncheck(p);
        }
    }
}

#[test]
fn test_max_heap() {
    let max_heap = Heap::new(vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1], true);
    let h1 = Heap::new(vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1], true);
    assert_eq!(max_heap, h1);
    let h2 = Heap::new(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1], true);
    assert_eq!(max_heap, h2);

    let max_heap = Heap::new(vec![16, 7, 10, 5, 1, 2], true);
    let h3 = Heap::new(vec![16, 7, 10, 5, 1, 2], true);
    assert_eq!(max_heap, h3);
    let h4 = Heap::new(vec![10, 7, 2, 5, 1], true).add_node(16);
    assert_eq!(&max_heap, &h4);

    let sort_h4 = h4.sort();
    assert_eq!(vec![1, 2, 5, 7, 10, 16], sort_h4);

    let min_vec = Heap::new(vec![9, 14, 8, 7, 16, 3, 2, 10, 4, 1], true).find_min_k(5);
    assert_eq!(vec![7, 3, 4, 1, 2], min_vec);
}

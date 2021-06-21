#[derive(PartialEq, Debug)]
pub struct Heap(Vec<i64>);

impl Heap {
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
        self.0.len()
    }
    fn get_uncheck(&self, i: usize) -> &i64 {
        self.0.get(i - 1).unwrap_or(&0)
    }
    fn swap_uncheck(&mut self, i: usize, j: usize) {
        self.0.swap(i - 1, j - 1)
    }
    fn max_shift_down_uncheck(&mut self, i: usize) {
        let l = Self::left(i);
        let r = Self::right(i);
        let mut larget = i;
        if l < self.size() && self.get_uncheck(l) > self.get_uncheck(i) {
            larget = l;
        }
        if r < self.size() && self.get_uncheck(r) > self.get_uncheck(larget) {
            larget = r;
        }
        if larget != i {
            self.swap_uncheck(larget, i);
            self.max_shift_down_uncheck(larget);
        }
    }
    pub fn as_max_heap(mut self) -> Self {
        for i in (1..(Self::parent(self.size()))).rev() {
            self.max_shift_down_uncheck(i);
        }
        self
    }
    pub fn new_max(slice: Vec<i64>) -> Self {
        Heap(slice).as_max_heap()
    }
    pub fn delete_node(mut self, i: usize) -> Self {
        if i == 0 || i > self.size() {
            return self;
        }
        self.swap_uncheck(i, self.size());
        self.0.pop();
        self.max_shift_down_uncheck(i);
        self
    }
    fn max_shift_up_uncheck(&mut self, i: usize) {
        if i == 0 || i == 1 {
            return;
        }
        let p = Self::parent(i);
        if self.get_uncheck(i) > self.get_uncheck(p) {
            self.swap_uncheck(i, p);
            self.max_shift_up_uncheck(p);
        }
    }
    pub fn add_node(mut self, v: i64) -> Self {
        self.0.push(v);
        self.max_shift_up_uncheck(self.size());
        self
    }
}

#[test]
fn test_max_heap() {
    let max_heap = Heap(vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
    let h1 = Heap(vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]).as_max_heap();
    assert_eq!(max_heap, h1);
    let h2 = Heap(vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1]).as_max_heap();
    assert_eq!(max_heap, h2);

    let max_heap = Heap(vec![16, 7, 10, 5, 1, 2]);
    let h3 = Heap(vec![16, 7, 10, 5, 1, 2]).as_max_heap();
    assert_eq!(max_heap, h3);
    let h4 = Heap(vec![10, 7, 2, 5, 1]).as_max_heap().add_node(16);
    assert_eq!(max_heap, h4);
}

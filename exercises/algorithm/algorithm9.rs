use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Clone, // 添加 Clone 特征约束
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone, // 添加 Clone 特征约束
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 使用默认值作为占位符
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.heapify_up(self.count);
    }

    fn heapify_up(&mut self, idx: usize) {
        let mut current_idx = idx;
        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx);
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        if right_idx <= self.count {
            if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
                left_idx
            } else {
                right_idx
            }
        } else {
            left_idx
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        let mut current_idx = idx;
        while self.children_present(current_idx) {
            let child_idx = self.smallest_child_idx(current_idx);
            if (self.comparator)(&self.items[child_idx], &self.items[current_idx]) {
                self.items.swap(current_idx, child_idx);
                current_idx = child_idx;
            } else {
                break;
            }
        }
    }
    
    // 删除堆顶元素并返回它
    pub fn remove(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        let top = self.items[1].clone(); // 获取堆顶元素
        self.items.swap(1, self.count); // 将最后一个元素放到堆顶
        self.items.pop(); // 删除最后一个元素
        self.count -= 1; // 减少元素数量
        self.heapify_down(1); // 向下调整堆
        Some(top)
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone, // 添加 Clone 特征约束
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone, // 添加 Clone 特征约束
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.remove() // 直接使用 remove 方法
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone, // 添加 Clone 特征约束
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone, // 添加 Clone 特征约束
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}

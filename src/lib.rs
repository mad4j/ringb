
pub struct RingB<T> {
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
    items: Vec<Option<T>>,
}

pub struct OverflowError;

pub const DEFAULT_BUFFER_CAPACITY: usize = 16;

impl<T> RingB<T> {

    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BUFFER_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {

        // allocate a buffer initialized at None
        let mut v = Vec::with_capacity(capacity);
        v.resize_with(capacity, || None);
        
        RingB { head: 0, tail: 0, size: 0, capacity, items: v }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self)-> bool {
        self.size == self.capacity    
    }

    pub fn enqueue(&mut self, item: T) {

        // discard oldest item
        if self.is_full() {
            let _ = self.dequeue();
        }

        // store item in the next slot
        self.items[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;
        
        // update buffer size
        debug_assert!(self.size < self.capacity);
        self.size += 1;
    }


    pub fn enqueue_or_overflow(&mut self, item: T) -> Result<(), OverflowError> {

        if self.is_full() {
            return Err(OverflowError);
        }

        // store item in the next slot
        self.items[self.tail] = Some(item);
        self.tail = (self.tail + 1) % self.capacity;

        // update buffer size
        debug_assert!(self.size < self.capacity);
        self.size += 1;

        Ok(())
    }


    pub fn dequeue(&mut self) -> Option<T> {

        // no items in buffer
        if self.is_empty() {
            return None;
        }

        // retrieve current item and leave None in its place
        let item = self.items[self.head].take();
        debug_assert!(item.is_some());

        // advance tail index
        self.head = (self.head + 1) % self.capacity;

        // update buffer size
        debug_assert!(self.size > 0);
        self.size -= 1;

        // return retrieved item
        item
    }

}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn new() {

        let b = RingB::<usize>::new();

        assert!(b.is_empty());
        assert!(!b.is_full());
        assert_eq!(b.head, 0);
        assert_eq!(b.tail, 0);
        assert_eq!(b.size(), 0);
        assert_eq!(b.capacity(), DEFAULT_BUFFER_CAPACITY);
    }

    #[test]
    fn with_capacity() {

        let b = RingB::<usize>::with_capacity(10);

        assert!(b.is_empty());
        assert!(!b.is_full());
        assert_eq!(b.head, 0);
        assert_eq!(b.tail, 0);
        assert_eq!(b.size(), 0);
        assert_eq!(b.capacity(), 10);
    }

    #[test]
    fn size() {

        let mut v = RingB::with_capacity(10);

        debug_assert_eq!(v.size(), 0);

        for i in 0..10 {
            v.enqueue(i);
            assert_eq!(v.size(), i+1);
        }

        for i in 0..5 {
            v.enqueue(i);
            assert_eq!(v.size(), v.capacity());
        }

        for i in 0..10 {
            let _ = v.dequeue();
            assert_eq!(v.size(), 10-i-1);
        }

        for _ in 0..5 {
            let _ = v.dequeue();
            assert_eq!(v.size(), 0);
        }

    }

    #[test]
    fn is_empty() {

        let mut b = RingB::new();
        assert!(b.is_empty());
        assert!(!b.is_full());
        assert!(b.size() == 0);
        assert!(b.head == 0);
        assert!(b.tail == 0);

        b.enqueue(10);
        assert!(!b.is_empty());
        assert!(!b.is_full());
        assert_eq!(b.size(), 1);
        assert_eq!(b.head, 0);
        assert_eq!(b.tail, 1);

        b.dequeue();
        assert!(b.is_empty());
    }

    #[test]
    fn enqueue() {
        let mut b = RingB::with_capacity(10);

        assert_eq!(b.capacity(), 10);
        assert_eq!(b.size(), 0);
        assert!(b.is_empty());
      

        for i in 0..10 {
            
            assert!(!b.is_full());

            b.enqueue(i);
            
            assert_eq!(b.capacity(), 10);
            assert_eq!(b.size(), i+1);
            assert!(!b.is_empty());
            
        }

        assert!(b.is_full());
    }


    #[test]
    fn simple() {
        let mut b = RingB::with_capacity(10);

        b.enqueue(2);
        let i = b.dequeue();

        assert_eq!(i, Some(2));
    }

    #[test]
    fn simple_string() {

        let mut b = RingB::with_capacity(10);

        let s = String::from("hello");
        b.enqueue(s);

        // s not more valid
        // let _ = s.is_ascii();

        let s = b.dequeue().unwrap();
        assert_eq!(s.cmp(&String::from("hello")), std::cmp::Ordering::Equal);

    }

    #[test]
    fn simple_string_ref() {

        let mut b = RingB::with_capacity(10);

        let s = String::from("hello");
        b.enqueue(&s);

        // s still valid
        let _ = s.is_ascii();

        let t = b.dequeue().unwrap();
        assert_eq!(t.cmp(&s), std::cmp::Ordering::Equal);

    }

}

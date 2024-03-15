use rand::Rng;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

const MAX_LEVEL: usize = 16;

struct Node<K, V> {
    key: Option<K>,
    value: Option<V>,
    forward: Vec<AtomicPtr<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn new(level: usize) -> Self {
        Self {
            key: None,
            value: None,
            forward: (0..level).map(|_| AtomicPtr::new(std::ptr::null_mut())).collect(),
        }
    }

    fn new_with_kv(key: K, value: V, level: usize) -> Self {
        Self {
            key: Some(key),
            value: Some(value),
            forward: (0..level).map(|_| AtomicPtr::new(std::ptr::null_mut())).collect(),
        }
    }
}

struct SkipList<K: Ord, V> {
    head: AtomicPtr<Node<K, V>>,
    max_level: AtomicUsize,
}

impl<K: Ord, V> SkipList<K, V> {
    fn new() -> Self {
        let head = AtomicPtr::new(Box::into_raw(Box::new(Node::new(MAX_LEVEL))));
        let tail = AtomicPtr::new(Box::into_raw(Box::new(Node::new(MAX_LEVEL))));

        unsafe {
            (*head.load(Ordering::Relaxed))
                .forward[0]
                .store(tail.load(Ordering::Relaxed), Ordering::Relaxed);
        }

        Self {
            head,
            max_level: AtomicUsize::new(1),
        }
    }

    fn insert(&self, key: K, value: V) {
        let mut update = vec![self.head.load(Ordering::Relaxed); MAX_LEVEL];
        let mut x = self.head.load(Ordering::Relaxed);

        for i in (0..self.max_level.load(Ordering::Relaxed)).rev() {
            unsafe {
                while let Some(next) = (*x).forward[i].load(Ordering::Relaxed).as_ref() {
                    if let Some(next_key) = &next.key {
                        if next_key < &key {
                            x = (*x).forward[i].load(Ordering::Relaxed);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            update[i] = x;
        }

        let level = random_level();
        let new_node = Box::new(Node::new_with_kv(key, value, level));
        let new_node_ptr = Box::into_raw(new_node);

        for i in 0..level {
            loop {
                let next = unsafe { (*update[i]).forward[i].load(Ordering::Relaxed) };
                unsafe { (*new_node_ptr).forward[i].store(next, Ordering::Relaxed) };

                if unsafe { (*update[i]).forward[i].compare_exchange(next, new_node_ptr, Ordering::Relaxed, Ordering::Relaxed).is_ok() } {
                    break;
                }
            }
        }

        self.max_level.fetch_max(level, Ordering::Relaxed);
    }

    fn find(&self, key: &K) -> Option<&V> {
        let mut x = self.head.load(Ordering::Relaxed);
        for i in (0..self.max_level.load(Ordering::Relaxed)).rev() {
            unsafe {
                while let Some(next) = (*x).forward[i].load(Ordering::Relaxed).as_ref() {
                    if let Some(next_key) = &next.key {
                        if next_key < key {
                            x = (*x).forward[i].load(Ordering::Relaxed);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        unsafe {
            x = (*x).forward[0].load(Ordering::Relaxed);
            if x.is_null() {
                None
            } else if let Some(x_key) = &(*x).key {
                if x_key == key {
                    (*x).value.as_ref()
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    fn remove(&self, key: &K) -> Option<V> {
        let mut update = vec![self.head.load(Ordering::Relaxed); MAX_LEVEL];
        let mut x = self.head.load(Ordering::Relaxed);

        for i in (0..self.max_level.load(Ordering::Relaxed)).rev() {
            unsafe {
                while let Some(next) = (*x).forward[i].load(Ordering::Relaxed).as_ref() {
                    if let Some(next_key) = &next.key {
                        if next_key < key {
                            x = (*x).forward[i].load(Ordering::Relaxed);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            update[i] = x;
        }

        unsafe {
            x = (*x).forward[0].load(Ordering::Relaxed);
            if x.is_null() || (*x).key.as_ref().unwrap() != key {
                None
            } else {
                for i in 0..self.max_level.load(Ordering::Relaxed) {
                    if (*update[i]).forward[i].load(Ordering::Relaxed) != x {
                        break;
                    }
                    (*update[i]).forward[i].store((*x).forward[i].load(Ordering::Relaxed), Ordering::Relaxed);
                }

                let removed_node = Box::from_raw(x);
                Some(removed_node.value.unwrap())
            }
        }
    }
}

fn random_level() -> usize {
    let mut level = 1;
    let mut rng = rand::thread_rng();
    while level < MAX_LEVEL && rng.gen::<f64>() < 0.5 {
        level += 1;
    }
    level
}

fn main() {
    let skiplist = SkipList::<i32, i32>::new();
    skiplist.insert(1, 10);
    skiplist.insert(2, 20);
    skiplist.insert(3, 30);

    if let Some(value) = skiplist.find(&2) {
        println!("Found: {}", value);
    } else {
        println!("Not found");
    }

    if let Some(value) = skiplist.remove(&2) {
        println!("Removed: {}", value);
    } else {
        println!("Not found");
    }

    if let Some(value) = skiplist.find(&2) {
        println!("Found: {}", value);
    } else {
        println!("Not found");
    }
}

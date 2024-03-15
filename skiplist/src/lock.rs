use rand::Rng;
use std::sync::{Arc, Mutex};

const MAX_LEVEL: usize = 16;

struct Node<K, V> {
    key: K,
    value: V,
    forward: Vec<Option<Arc<Mutex<Node<K, V>>>>>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V, level: usize) -> Self {
        Self {
            key,
            value,
            forward: vec![None; level],
        }
    }
}

struct SkipList<K: Ord, V> {
    head: Arc<Mutex<Node<K, V>>>,
    max_level: usize,
}

impl<K: Ord + Copy, V: Copy> SkipList<K, V> {
    fn new() -> Self {
        let head = Arc::new(Mutex::new(Node::new(
            std::i32::MIN as K,
            std::i32::MIN as V,
            MAX_LEVEL,
        )));
        Self {
            head,
            max_level: 1,
        }
    }

    fn insert(&mut self, key: K, value: V) {
        let mut update = vec![self.head.clone(); MAX_LEVEL];
        let mut x = self.head.clone();

        for i in (0..self.max_level).rev() {
            let mut x_guard = x.lock().unwrap();
            while let Some(ref next) = x_guard.forward[i] {
                let next_guard = next.lock().unwrap();
                if next_guard.key < key {
                    x = next.clone();
                    x_guard = x.lock().unwrap();
                } else {
                    break;
                }
            }
            drop(x_guard);
            update[i] = x.clone();
        }

        let level = random_level();
        let new_node = Arc::new(Mutex::new(Node::new(key, value, level)));

        for i in 0..level {
            let mut update_guard = update[i].lock().unwrap();
            new_node.lock().unwrap().forward[i] = update_guard.forward[i].clone();
            update_guard.forward[i] = Some(new_node.clone());
        }

        if level > self.max_level {
            self.max_level = level;
        }
    }

    fn find(&self, key: &K) -> Option<V> {
        let mut x = self.head.clone();
        for i in (0..self.max_level).rev() {
            let mut x_guard = x.lock().unwrap();
            while let Some(ref next) = x_guard.forward[i] {
                let next_guard = next.lock().unwrap();
                if next_guard.key < *key {
                    x = next.clone();
                    x_guard = x.lock().unwrap();
                } else {
                    break;
                }
            }
        }

        let x_guard = x.lock().unwrap();
        if x_guard.key == *key {
            Some(x_guard.value)
        } else {
            None
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
    let mut skiplist = SkipList::<i32, i32>::new();
    skiplist.insert(1, 10);
    skiplist.insert(2, 20);
    skiplist.insert(3, 30);

    if let Some(value) = skiplist.find(&2) {
        println!("Found: {}", value);
    } else {
        println!("Not found");
    }
}

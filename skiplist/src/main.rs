use core::f64;
use std::{
    error::Error,
    fmt::{Debug, Display},
    ptr::null_mut,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};

use rand::Rng;

const P: f64 = 0.5; //控制升入上一层的概率
const MAX_LEVEL: usize = 16;
#[derive(Debug)]
struct Node<K: Display, V: Display> {
    key: Option<K>,
    value: Option<V>,
    forward: Vec<AtomicPtr<Node<K, V>>>, //代表每一层的下一个值
}

impl<K: Display, V: Display> Node<K, V> {
    pub fn new(level: usize) -> Self {
        Node {
            key: None,
            value: None,
            forward: (0..level).map(|_| AtomicPtr::new(null_mut())).collect(),
        }
    }
    pub fn node(level: usize, k: K, v: V) -> Self {
        Node {
            key: Some(k),
            value: Some(v),
            forward: (0..level).map(|_| AtomicPtr::new(null_mut())).collect(),
        }
    }
}

struct SkipList<K: Display, V: Display>
where
    K: Ord,
{
    head: AtomicPtr<Node<K, V>>, //做为哨兵结点，方便插入和删除
    max_height: AtomicUsize,
}

impl<K: Ord + Display, V: Display> SkipList<K, V> {
    pub fn new() -> Self {
        let head = AtomicPtr::new(Box::into_raw(Box::new(Node::new(MAX_LEVEL))));
        let tail = AtomicPtr::new(Box::into_raw(Box::new(Node::new(MAX_LEVEL))));
        unsafe {
            (*head.load(Ordering::Acquire)).forward[0]
                .store(tail.load(Ordering::Acquire), Ordering::Release);
        }
        Self {
            head,
            max_height: AtomicUsize::new(1),
        }
    }

    pub fn insert(&mut self, target_key: K, target_value: V) -> Result<(), Box<dyn Error>>
    where
        K: Ord + Debug,
        V: Debug,
    {
        let mut index = self.head.load(Ordering::Acquire);
        let mut insert_pos = vec![self.head.load(Ordering::Acquire); MAX_LEVEL];
        for i in (0..self.max_height.load(Ordering::Acquire)).rev() {
            unsafe {
                while let Some(next) = (*index).forward[i].load(Ordering::Relaxed).as_ref() {
                    if let Some(key) = &next.key {
                        if &target_key > key {
                            index = (*index).forward[i].load(Ordering::Acquire);
                        } else if &target_key == key {
                            let node = &mut *(*index).forward[i].load(Ordering::Acquire);
                            node.value = Some(target_value);
                            return Ok(());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                insert_pos[i] = index;
            }
        }
        let level = random_level();
        let new_node = Box::into_raw(Box::new(Node::node(level, target_key, target_value)));
        //插入每层的结点
        for i in 0..level {
            unsafe {
                loop {
                    let next_node = (*insert_pos[i]).forward[i].load(Ordering::Acquire);
                    (*new_node).forward[i].store(next_node, Ordering::Release);
                    if (*insert_pos[i]).forward[i]
                        .compare_exchange(next_node, new_node, Ordering::AcqRel, Ordering::Acquire)
                        .is_ok()
                    {
                        break;
                    }
                }
            }
        }
        //更新最高层
        self.max_height.fetch_max(level, Ordering::Relaxed);
        Ok(())
    }

    pub fn find(&mut self, target_key: K) -> Option<&V> {
        let mut node = self.head.load(Ordering::Acquire);
        let level = self.max_height.load(Ordering::Relaxed);
        for i in (0..level).rev() {
            unsafe {
                while let Some(next) = (*node).forward[i].load(Ordering::Acquire).as_ref() {
                    if let Some(ref key) = next.key {
                        if target_key > *key {
                            node = (*node).forward[i].load(Ordering::Acquire);
                        } else if target_key == *key {
                            return next.value.as_ref();
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        None
    }

    pub fn delete(&mut self, target_key: K) -> Result<(), Box<dyn Error>> {
        let mut node = self.head.load(Ordering::Acquire);
        let level = self.max_height.load(Ordering::Acquire);
        let mut remove_pos = vec![node; MAX_LEVEL];
        for i in (0..level).rev() {
            unsafe {
                while let Some(next) = (*node).forward[i].load(Ordering::Acquire).as_ref() {
                    if let Some(ref key) = next.key {
                        if key < &target_key {
                            node = (*node).forward[i].load(Ordering::Acquire);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                remove_pos[i] = node;
            }
        }
        unsafe {
            node = (*remove_pos[0]).forward[0].load(Ordering::Acquire);
            for i in 0..self.max_height.load(Ordering::Acquire) {
                if node.is_null() {
                    break;
                } else if (*node).key.as_ref().unwrap() != &target_key {
                    break;
                }
                (*remove_pos[i]).forward[i].store(
                    (*node).forward[i].load(Ordering::Acquire),
                    Ordering::Release,
                );
            }
            while self.max_height.load(Ordering::Acquire) > 1
                && (*self.head.load(Ordering::Acquire)).forward
                    [self.max_height.load(Ordering::Acquire) - 1]
                    .load(Ordering::Acquire)
                    .is_null()
            {
                self.max_height.fetch_sub(1, Ordering::AcqRel);
            }
            let delete_node = Box::from_raw(node);
            // drop(delete_node);
        }
        Ok(())
    }
}
impl<K: Ord + Display, V: Display> Display for SkipList<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Max_height = {}", MAX_LEVEL)?;
        let max_digits = format!("{}", MAX_LEVEL).len();
        writeln!(f, "Current SkipList:")?;
        for i in (0..self.max_height.load(Ordering::Acquire)).rev() {
            write!(f, "Level {:>width$}: ", i, width = max_digits)?;
            unsafe {
                let mut index =
                    (*self.head.load(Ordering::Relaxed)).forward[i].load(Ordering::Acquire);
                while let Some(node) = index.as_ref() {
                    if let (Some(key), Some(value)) = (&node.key, &node.value) {
                        write!(f, "({key},{value}) -> ")?;
                    }
                    index = node.forward[i].load(Ordering::Acquire);
                }
                write!(f, "Node")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
fn random_level() -> usize {
    let mut level = 1;
    let mut r = rand::thread_rng();
    while level < MAX_LEVEL && r.gen_range(0.0..1.0) < P {
        level += 1;
    }
    level
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut skiplist = SkipList::new();
    for i in 0..10 {
        let task = tokio::spawn(async move  {
            skiplist.insert(i, i * 10);
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {}

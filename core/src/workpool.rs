use crossbeam::deque::{Injector, Steal, Stealer, Worker};
use crossbeam::queue::{ArrayQueue, SegQueue};
use crossbeam::scope;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct WorkPool<T> {
    global: Arc<Injector<T>>,
    local: Worker<T>,
    stealers: Arc<RwLock<Vec<Stealer<T>>>>,
}

impl<T> WorkPool<T> {
    pub fn new() -> Self {
        let local = Worker::new_fifo();
        let stealers = Arc::new(RwLock::new(vec![local.stealer()]));

        Self {
            local,
            stealers,
            global: Arc::new(Injector::new()),
        }
    }

    pub fn get_work(&self) -> Option<T> {
        if let Some(work) = self.local.pop() {
            return Some(work);
        }

        match self.global.steal_batch_and_pop(&self.local) {
            Steal::Success(work) => return Some(work),
            Steal::Retry => return self.get_work(),
            Steal::Empty => {
                let mut retry = false;
                while retry {
                    let g = self.stealers.read().expect("Poisoned work stealers");
                    for s in g.iter() {
                        match s.steal_batch_and_pop(&self.local) {
                            Steal::Success(work) => return Some(work),
                            Steal::Retry => retry = true,
                            _ => {}
                        }
                    }
                }
                None
            }
        }
    }

    pub fn push_work(&self, task: T) {
        self.global.push(task)
    }

    pub fn global(&self) -> Arc<Injector<T>> {
        self.global.clone()
    }
}

impl<T> Clone for WorkPool<T> {
    fn clone(&self) -> Self {
        let local = Worker::new_fifo();
        self.stealers
            .write()
            .expect("Poisoned work stealers")
            .push(local.stealer());
        Self {
            local,
            global: self.global.clone(),
            stealers: self.stealers.clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const THREADS: usize = 2;
    const MESSAGES: usize = 50;

    // #[test]
    // fn it_works() {
    //     let wp = WorkPool::<String>::new();
    //     let mut wps = Vec::<&WorkPool<String>>::new();
    //     scope(move |s| {
    //         for i in 0..THREADS {
    //             let wp = wp.clone();
    //             wps.push(&wp);
    //             s.spawn(move |_| {
    //                 for y in 0..MESSAGES {
    //                     wp.push_work(format!("push:{}-{}", thread::current().name().unwrap(), y));
    //                 }
    //             });
    //         }
    //
    //         // for wb in wps.iter() {
    //         //     for i in 0..THREADS {
    //         //         s.spawn(move |_| {
    //         //             for y in 0..MESSAGES {
    //         //                 println!(
    //         //                     "push:{}-{}",
    //         //                     thread::current().name().unwrap(),
    //         //                     wp.get_work().unwrap()
    //         //                 );
    //         //             }
    //         //         });
    //         //         // println!("{}", i);
    //         //     }
    //         // }
    //     })
    //     .unwrap();
    // }

    // #[test]
    // fn mpsc() {
    //     let wp = WorkPool::<String>::new();
    //     let mut wps = Vec::<WorkPool<String>>::new();
    //     let q = SegQueue::<&WorkPool<String>>::new();
    //
    //     scope(|scope| {
    //         for _ in 0..THREADS {
    //             let wp = wp.clone();
    //             q.push(&wp);
    //             scope.spawn(move |_| {
    //                 for y in 0..MESSAGES {
    //                     wp.push_work(format!("push:{}-{}", thread::current().name().unwrap(), y));
    //                 }
    //             });
    //         }
    //     })
    //     .unwrap();
    //
    //     // thread::spawn(move || {
    //     //     for i in 0..THREADS {
    //     //         let w = wp.clone();
    //     //         for y in 0..MESSAGES {
    //     //             w.push_work(format!("push:{}-{}", thread::current().name().unwrap(), y));
    //     //         }
    //     //         wps.push(w);
    //     //     }
    //     // });
    //
    //     // scope(move |s| {
    //
    //     // for wb in wps.iter() {
    //     //     for y in 0..MESSAGES {
    //     //         println!("get:{}", wp.get_work().unwrap());
    //     //     }
    //     // }
    // }
}

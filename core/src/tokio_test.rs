use dashmap::DashMap;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::oneshot;
use tokio::task;

#[derive(Debug)]
enum Command {
    GET { key: String },
    SET { key: String, val: String },
}

type Response<T> = oneshot::Sender<T>;

#[derive(Debug)]
enum Command_2 {
    GET {
        key: String,
        resp_sender: Response<String>,
    },
    SET {
        key: String,
        val: String,
        resp_sender: Response<String>,
    },
}

async fn do_db(db: &Arc<Mutex<HashMap<i32, i32>>>) {
    for i in 0..4 {
        let db = db.clone();
        task::spawn(async move {
            let mut map = db.lock().unwrap();
            map.insert(i, i);
            println!("{}", thread::current().name().unwrap());
        })
        .await
        .unwrap();
    }
}

async fn do_db2(db: &Arc<DashMap<i32, i32>>) {
    for i in 0..4 {
        let db = db.clone();
        task::spawn(async move {
            db.insert(i, i);
            println!("{}", thread::current().name().unwrap());
        })
        .await
        .unwrap();
    }
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("Hello");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokio_test::Command::{GET, SET};
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test1() {
        let handle = tokio::spawn(async {
            // 做一些异步的工作
            1
        });

        // 作一些其它的工作

        let out = handle.await.unwrap();
        println!("GOT {}", out);

        let v = vec![1, 2, 3];

        let h = task::spawn(async move {
            println!("Here's a vec: {:?}", v);
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_db() {
        let db = Arc::new(Mutex::new(HashMap::new()));
        do_db(&db).await;

        for (k, v) in db.lock().unwrap().iter() {
            println!("{}:{}", k, v)
        }
    }

    #[tokio::test]
    async fn test_db2() {
        let db = Arc::new(DashMap::new());
        do_db2(&db).await;

        db.iter().for_each(|x| {
            println!("{}:{}", x.key(), x.value());
        });
    }

    #[tokio::test]
    async fn test_mpsc1() {
        let (tx, mut rx) = mpsc::channel(32);
        let tx2 = tx.clone();

        tokio::spawn(async move {
            tx.send("sending from first handle").await;
        });

        tokio::spawn(async move {
            tx2.send("sending from second handle").await;
        });

        while let Some(message) = rx.recv().await {
            println!("GOT = {}", message);
        }
    }

    #[tokio::test]
    async fn test_mpsc2() {
        let (s, mut r) = mpsc::channel(32);
        let s2 = s.clone();

        let handler1 = tokio::spawn(async move {
            let cmd = Command::GET {
                key: String::from("nauu"),
            };
            println!("send cmd get: {:?}", cmd);
            s.send(cmd).await;
        })
        .await;

        let handler2 = tokio::spawn(async move {
            let cmd = Command::SET {
                key: String::from("nauu"),
                val: "gogogo".to_string(),
            };
            println!("send cmd set: {:?}", cmd);
            s2.send(cmd).await;
        })
        .await;

        let manager = tokio::spawn(async move {
            while let Some(cmd) = r.recv().await {
                match cmd {
                    GET { key } => {
                        println!("get key: {}", key);
                    }
                    SET { key, val } => {
                        println!("set key: {},{}", key, val);
                    }
                }
            }
        })
        .await;
    }

    #[tokio::test]
    async fn test_mpsc3() {
        let (s, mut r) = mpsc::channel(32);
        let s2 = s.clone();

        let handler1 = tokio::spawn(async move {
            let (resp_sender, resp_recver) = oneshot::channel();
            let cmd = Command_2::GET {
                key: String::from("nauu"),
                resp_sender: resp_sender,
            };
            println!("send cmd get: {:?}", cmd);
            s.send(cmd).await.unwrap();

            //等待响应
            let res = resp_recver.await;
            println!("handler1 GOT = {:?}", res);
        });

        let handler2 = tokio::spawn(async move {
            let (resp_sender, resp_recver) = oneshot::channel();
            let cmd = Command_2::SET {
                key: String::from("nauu"),
                val: "gogogo".to_string(),
                resp_sender: resp_sender,
            };
            println!("send cmd set: {:?}", cmd);
            s2.send(cmd).await;

            //等待响应
            let res = resp_recver.await;
            println!("handler2 GOT = {:?}", res);
        });

        let manager = tokio::spawn(async move {
            while let Some(cmd) = r.recv().await {
                match cmd {
                    Command_2::GET { key, resp_sender } => {
                        println!("get key: {}", key);
                        let _ = resp_sender.send(format!("res get key: {}", key));
                    }
                    Command_2::SET {
                        key,
                        val,
                        resp_sender,
                    } => {
                        println!("set key: {},{}", key, val);
                        let _ = resp_sender.send(format!("res set key: {}", key));
                    }
                }
            }
        })
        .await;
    }

    #[tokio::test]
    async fn test_delay() {
        let when = Instant::now() + Duration::from_millis(10);
        let future = Delay { when };
        let out = future.await;
        println!("{}", out);
    }
}

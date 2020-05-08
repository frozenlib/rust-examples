use std::{task::Poll, thread::sleep, time::Duration, future::Future, sync::Arc};
use tokio::{sync::Mutex, task::spawn};

// Mutex::lock()の戻り値のFutureを破棄すると、そのFutureを再開するためのWaker::wakeは呼び出されないという例

#[tokio::main]
async fn main() {
    let m = Arc::new(Mutex::new(()));

    let m1 = m.clone();
    let h1 = spawn(async move {
        let _guard = m1.lock().await;
        println!("sleep start");
        sleep(Duration::from_secs(5));
        println!("sleep end");
    });
    sleep(Duration::from_secs(1));

    // wakeの前にFutureを破棄する場合
    let h2 = spawn(BadMutexFuture(m.clone()));

    // wakeの前にFutureを破棄しない場合
    let h3 = spawn(async move {
        let _guard = m.lock().await;
        println!("lock succeeded");
    });

    h1.await.unwrap();
    h2.await.unwrap();
    h3.await.unwrap();
}

struct BadMutexFuture(Arc<Mutex<()>>);

impl Future for BadMutexFuture {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match Box::pin(self.get_mut().0.lock()).as_mut().poll(cx) {
            Poll::Ready(_) => {
                println!("ready");
                Poll::Ready(())
            }
            Poll::Pending => {
                println!("pending");
                Poll::Pending
            }
        }
    }
}

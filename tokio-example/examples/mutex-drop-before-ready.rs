use std::task::Context;
use std::{future::Future, pin::Pin, sync::Arc, task::Poll, thread::sleep, time::Duration};
use tokio::{
    sync::{Mutex, MutexGuard},
    task::spawn,
};

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
    let m3 = m.clone();
    let h3 = spawn(async move {
        let _guard = m3.lock().await;
        println!("lock succeeded");
    });

    // wakeの前にFutureを破棄しない場合
    let h4 = spawn(DangerousMutexFuture::new(m.clone()));

    h1.await.unwrap();
    h2.await.unwrap();
    h3.await.unwrap();
    h4.await.unwrap();
}

struct BadMutexFuture(Arc<Mutex<()>>);

impl Future for BadMutexFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Box::pin(self.get_mut().0.lock()).as_mut().poll(cx) {
            Poll::Ready(_) => {
                println!("ready bad");
                Poll::Ready(())
            }
            Poll::Pending => {
                println!("pending bad");
                Poll::Pending
            }
        }
    }
}

struct DangerousMutexFuture {
    mutex: Arc<Mutex<()>>,

    // 正しいライフタイムを表現できないため、仕方なくstaticライフタイムを使っている
    // これがUBを引き起こさないか心配
    fut: Option<Pin<Box<dyn Future<Output = MutexGuard<'static, ()>> + Send>>>,
}

impl Drop for DangerousMutexFuture {
    fn drop(&mut self) {
        // futはmutexを参照しているので、mutexより先にfutが解放されるとヤバい
        self.fut.take();
    }
}

impl DangerousMutexFuture {
    fn new(mutex: Arc<Mutex<()>>) -> Self {
        Self { mutex, fut: None }
    }
}

impl Future for DangerousMutexFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let s = self.get_mut();
        let fut = &mut s.fut;
        if fut.is_none() {
            let mutex = &s.mutex as &Mutex<()> as *const Mutex<()>;
            unsafe {
                *fut = Some(Box::pin((*mutex).lock()));
            }
        }
        match fut.as_mut().unwrap().as_mut().poll(cx) {
            Poll::Ready(_) => {
                fut.take();
                println!("ready dangerous");
                Poll::Ready(())
            }
            Poll::Pending => {
                println!("pending dangerous");
                Poll::Pending
            }
        }
    }
}

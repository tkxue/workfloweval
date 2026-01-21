use super::*;

#[derive(Clone)]
pub struct AsyncCondVar {
    inner: ArcState<Vec<async_oneshot::Sender<()>>>,
}

impl AsyncCondVar {
    pub fn new() -> AsyncCondVar {
        AsyncCondVar { inner: Default::default() }
    }

    pub async fn wait_on(&self) {
        let (send, recv) = async_oneshot::oneshot::<()>();
        self.inner.update(move |x| x.push(send));
        let _ = recv.await;
    }

    pub fn notify(&self) {
        let v = self.inner.update(|x| std::mem::replace(x, vec![]));
        for mut x in v.into_iter() {
            let _ = x.send(());
        }
    }
}

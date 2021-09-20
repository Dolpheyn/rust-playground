use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use tracing::info;

pub struct DumbFuture {}

impl Future for DumbFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        info!("Hello from a dumb future!");
        Poll::Ready(())
    }
}

#![allow(unused)]

use crossfire::mpsc::Array;
use crossfire::{AsyncRx, MAsyncTx, RecvError, SendError, TryRecvError, TrySendError};

#[derive(Clone)]
pub struct Tx<T>
where
	T: Send + 'static,
{
	inner: MAsyncTx<Array<T>>,
}

pub struct Rx<T>
where
	T: Send + 'static,
{
	inner: AsyncRx<Array<T>>,
}

/// creates a new bounded mpsc channel with the given capacity and returns the sender and receiver.
fn new_mpsc_bounded<E>() -> (Tx<E>, Rx<E>)
where
	E: Send + 'static,
{
	let (ctx, crx) = crossfire::mpsc::bounded_async::<E>(10_000);
	let tx = Tx { inner: ctx };
	let rx = Rx { inner: crx };
	(tx, rx)
}

// region:    --- Implementation

impl<T> Tx<T>
where
	T: Send + 'static,
{
	pub async fn send(&self, msg: T) -> Result<(), SendError<T>>
	where
		T: Unpin,
	{
		self.inner.send(msg).await
	}

	pub fn send_sync(&self, msg: T) -> Result<(), SendError<T>> {
		let tx = self.inner.clone().into_blocking();
		tx.send(msg)
	}

	pub fn try_send(&self, msg: T) -> Result<(), TrySendError<T>> {
		self.inner.try_send(msg)
	}
}

impl<T> Rx<T>
where
	T: Send + 'static,
{
	pub async fn recv(&self) -> Result<T, RecvError> {
		self.inner.recv().await
	}

	pub fn try_recv(&self) -> Result<T, TryRecvError> {
		self.inner.try_recv()
	}
}

// endregion: --- Implementation

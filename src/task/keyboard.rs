use crossbeam_queue::ArrayQueue;
use crate::println;
use crate::print;
use core::task::{Poll, Context};
use futures_util::stream::Stream;
use futures_util::stream::StreamExt;
use futures_util::task::AtomicWaker;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use lazy_static::lazy_static;

lazy_static! {
    static ref SCANCODE_QUEUE: ArrayQueue<u8> = ArrayQueue::new(100);
    static ref WAKER: AtomicWaker = AtomicWaker::new();
}

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(_) = SCANCODE_QUEUE.push(scancode) {
        WAKER.wake();
    } else {
        println!("WARNING: scancode queue full; dropping keyboard input");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        if let Some(scancode) = SCANCODE_QUEUE.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(cx.waker());
        match SCANCODE_QUEUE.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(ScancodeSet1::new(), layouts::Us104Key,
        HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}

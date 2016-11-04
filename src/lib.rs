//Copyright 2016 William Cody Laeder
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.

//!Multi-Sender Multi-Receiver First In First Out Channel
//!
//!This is an extremely simple FIFO algorithm. It is very fast, and very simple
//!The sender(s) do not check if there is anything to receive their data
//!and the receivers do not check if there is anything to send data to them.
//!
//!It uses completely safe code.
//!
//!Credit goes to Dmitry Vyukov. I've reimplemented his algorithm in Rust.
//!The orginal source code can be found
//![here](http://www.1024cores.net/home/lock-free-algorithms/queues/bounded-mpmc-queue)
//!
//!Do not store values inhernally that are larger than
//!240bytes. This can cause internal cache invalidation which will
//!slow down the algorithm.
//!

extern crate crossbeam;
use crossbeam::mem::CachePadded;
use std::sync::atomic::{AtomicUsize,Ordering};
use std::mem;

const REX: Ordering = Ordering::Relaxed;
const INV: usize = ::std::usize::MAX;

struct Cell<T: Sized> {
    pub sequ: AtomicUsize,
    pub data: Option<T>
}


///holds the queue stuff
pub struct MSMRBoundedQueue<T: Sized+'static> {
    buffer: Vec<CachePadded<Cell<T>>>,
    enqueue: AtomicUsize,
    dequeue: AtomicUsize,
    pad: [u8;48]
}
impl<T: Sized+'static> MSMRBoundedQueue<T> {
    ///constructor
    fn new(size: usize) -> MSMRBoundedQueue<T> {
        //allocate vector
        let mut v = Vec::<CachePadded<Cell<T>>>::with_capacity(size);
        //push each item
        for i in 0..size {
            v.push(CachePadded::new(Cell {
                sequ: AtomicUsize::new(i),
                data: None
            }));
        }
        MSMRBoundedQueue {
            buffer: v,
            enqueue: AtomicUsize::new(INV),
            dequeue: AtomicUsize::new(INV),
            pad: [0u8;48]
        }
    }
    ///enqueue item
    ///returns error on full
    fn enqueue(&mut self, data: T) -> Result<(),()> {
        let pos = self.enqueue.load(REX);
        if pos == INV {
            return Err(());
        }
        let mut x = CachePadded::new(Cell {
            sequ: AtomicUsize::new(0),
            data: None
        });
        loop {

}


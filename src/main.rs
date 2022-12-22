#![allow(unused_macros)]
#![warn(dead_code)]

use std::future::Future;
use std::mem::size_of;
use std::ops::Add;
use std::pin::Pin;
use std::sync::mpsc::{channel, Sender};
use std::task::{Context, Poll, Waker};

#[derive(Clone, Copy, Debug)]
struct Foo(usize);

impl Foo {
    fn new() -> Self {
        Foo(0)
    }
}

#[derive(Copy, Clone, Debug)]
struct Una<'foo> {
    foo: &'foo [Foo],
}

fn ab(b: String) -> String {
    b
}

fn foes(v: Foo) -> (Foo, Foo) {
    (v, v)
}
fn borrow(s: &str) -> &str {
    &s[..8]
}

trait Bar: Copy {
    fn foo(&self) -> usize;
}

impl Bar for Foo {
    fn foo(&self) -> usize {
        self.0
    }
}

struct Foo2<'a>(&'a str);

trait Bar2<'a>: 'a {}

impl<'a> Bar2<'a> for Foo2<'a> {}
impl<'a> Bar2<'a> for Foo {}

trait General {
    fn say_foo() {
        println!("foo");
    }

    fn say_bar(&self) {
        println!("bar");
    }
}

impl<T: Bar> General for T {}

trait MoreGen {
    fn say() {
        println!("Mi type {:?}", std::any::type_name::<Self>())
    }
}

impl<T> MoreGen for T {}

macro_rules! foo {
    ($_self:ident) => {};
}

struct FooFut<const MAX: usize> {
    state: String,
    // whats need here?
    channel: Sender<()>,
}

impl<const MAX: usize> Future for FooFut<MAX> {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state.len() == MAX {
            return Poll::Ready(self.state.len());
        }
        println!("step {}", self.state);
        self.state.push('c');
        // whats need here?
        let _ = self.channel.send(());
        Poll::Pending
    }
}

fn main() {
    let mut vec_una = vec![Foo::new(), Foo::new()];
    vec_una.push(Foo::new());
    let una = Una { foo: &vec_una };
    // I can do this?
    // vec_una.push(Foo::new());
    println!("{:?}", una);
    let foo = Foo::new();
    drop(foo);
    drop(foo);
    // can use `foo` here ?

    let a = String::new();
    let c = ab(a);
    // can use `a` here ?
    {
        c
    };
    // can use `c` here ?

    Foo::say_foo();
    usize::say();
    String::say();

    println!("Owned {}", size_of::<String>());
    println!("Borrowed {}", size_of::<&str>());
    println!(
        "Pointer {} == {}",
        size_of::<*const Foo>(),
        size_of::<usize>()
    );

    // Async
    // let (tx, rx) = channel::<()>();
    // let th2 = std::thread::spawn(move || loop {
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    //     match rx.recv() {
    //         Ok(_waiter) => {}
    //         Err(_) => {
    //             break;
    //         }
    //     }
    // });
    // println!(
    //     "Future {}",
    //     futures::executor::block_on(FooFut::<10> {
    //         state: String::new(),
    //         channel: tx,
    //     })
    // );
    //
    // th2.join().expect("thread 2 join correctly");
}

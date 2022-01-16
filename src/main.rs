use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char)
}

struct Progress<T, U> {
    iterator: T,
    i: usize,
    bound: U,
}

trait ProgressDisplay: Sized{
    fn display<T>(&self, progress: &Progress<T, Self>);
}

impl ProgressDisplay for Unbounded{
    fn display<T>(&self, progress: &Progress<T, Self>) {
        println!("{}", "*".repeat(progress.i))
    }
}

impl ProgressDisplay for Bounded{
    fn display<T>(&self, progress: &Progress<T, Self>) {
       println!("{}{}{}{}",
                    self.delims.0,
                    "*".repeat(progress.i),
                    " ".repeat(self.bound - progress.i ),
                    self.delims.1,
        )
    }
}

impl<T> Progress<T, Unbounded> {
    pub fn new(iterator: T) -> Self {
        Progress {
            iterator,
            i: 0,
            bound: Unbounded,
        }
    }
}

impl<T> Progress<T, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

impl<T> Progress<T, Unbounded>
where T: ExactSizeIterator
{
    pub fn with_bound(mut self) -> Progress<T, Bounded> {
        let bound = Bounded {
            bound: self.iterator.len(),
            delims: ('[',']')
        };
        Progress {
            i: self.i,
            iterator: self.iterator,
            bound
        }
    }
}

impl<T, U> Iterator for Progress<T, U>
where T: Iterator,
      U: ProgressDisplay
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        self.bound.display(&self);
        self.i += 1;
        self.iterator.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<T> ProgressIteratorExt for T {
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let brackets: (char, char) = ('<', '>');

    //for n in (0 ..).progress() {
    //for n in (0 ..).progress().with_delims(brackets) {
    //    expensive_calculation(&n);
    //}

    let v = vec![1, 2, 3];
    for n in v.iter().progress().with_bound().with_delims(brackets) {
        expensive_calculation(n);
    }
}

use std::collections::BinaryHeap;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
enum Task {
  A,
  B,
  C,
}

fn main() {
  let mut queue = BinaryHeap::new();

  queue.push((1, Task::A));
  queue.push((1, Task::B));
  queue.push((2, Task::C));
  queue.push((0, Task::C));
  queue.push((1, Task::A));
  queue.push((1, Task::C));

  while let Some((priority, task)) = queue.pop() {
    println!("{priority} {task:?}");
  }
}

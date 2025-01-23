// In this exercise, we are given a `Vec` of `u32` called `numbers` with values
// ranging from 0 to 99. We would like to use this set of numbers within 8
// different threads simultaneously. Each thread is going to get the sum of
// every eighth value with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, …
// The second thread (offset 1), will sum 1, 9, 17, …
// The third thread (offset 2), will sum 2, 10, 18, …
// …
// The eighth thread (offset 7), will sum 7, 15, 23, …
//
// Each thread should own a reference-counting pointer to the vector of
// numbers. But `Rc` isn't thread-safe. Therefore, we need to use `Arc`.
//
// Don't get distracted by how threads are spawned and joined. We will practice
// that later in the exercises about threads.
// 在这个练习中，我们得到一个名为 “numbers” 的 “u32” 的 “vec”，其值为
// 从0到99。我们想在8内使用这组数字
// 同时不同的线程。每个线程将获得的总和
// 每八个值有一个偏移量。
//
// 第一个线程 (偏移量0)，将总和为0，8，16，…
// 第二个线程 (偏移量1)，将总和1，9，17，…
// 第三个线程 (偏移2)，将总和2，10，18，…
// …
// 第八个线程 (偏移7)，将总和7，15，23，…
//
// 每个线程都应该拥有一个指向向量的引用计数指针
// 数字。但是 'Rc' 不是线程安全的。因此，我们需要使用 'Arc'。
//
// 不要被线程如何产生和加入而分心。我们将练习
// 在后面的练习中关于线程。

// Don't change the lines below.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Define `shared_numbers` by using `Arc`.
    // let shared_numbers = ???;
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Define `child_numbers` using `shared_numbers`.
        // let child_numbers = ???;
        let child_numbers = shared_numbers.clone();
        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}

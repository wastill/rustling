// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises, the code will fail to compile when there are Clippy
// warnings. Check Clippy's suggestions from the output to solve the exercise.

// Clippy工具是一个lint的集合来分析你的代码，这样你就可以
// 抓住常见错误并改进Rust代码。
//
// 对于这些练习，当有Clippy时，代码将无法编译
// 警告。从输出中检查Clippy的建议以解决练习。

fn main() {
    // TODO: Fix the Clippy lint in this line.
    let pi: f32 = std::f32::consts::PI;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}

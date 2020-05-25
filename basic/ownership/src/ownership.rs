// 移动
pub fn what_is_ownership() {
    { // Scope:
        let s = String::from("hello"); // 从此处起，s是有效的
        println!("{}", s);
        // 此作用域已结束，
    } // 这是一个将 String 需要的内存返回给操作系统的很自然的位置：
    // 当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，
    // 在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。


    { // Move:
        let s1 = String::from("hello");
        let s2 = s1;
        println!("s2: {}", s2);
        // 上面的例子可以解读为 s1 被 移动 到了 s2 中。
        // 因为只有 s2 是有效的，当其离开作用域，它就释放自己的内存，完毕。
        // 另外，这里还隐含了一个设计选择：Rust 永远也不会自动创建数据的 “深拷贝”。
        // 因此，任何 自动 的复制可以被认为对运行时性能影响较小。
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1={}, s2={}", s1, s2);
        // 这里堆上的数据 确实 被复制了。
        // 当出现 clone 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源。
        // 你很容易察觉到一些不寻常的事情正在发生。
    }

    // 如下是一些 Copy 的类型：
    //  所有整数类型，比如 u32。
    //  布尔类型，bool，它的值是 true 和 false。
    //  所有浮点数类型，比如 f64。
    //  字符类型，char。
    //  元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

    { // 所有权与函数
        let s = String::from("hello"); // s 进入作用域
        takes_ownership(s); // s 的值移动到函数里 ...
                                      // ... 所以到这里不再有效

        let x = 5;          // x 进入作用域
        makes_copy(x);                  // x 应该移动函数里，
        // 但 i32 是 Copy 的，所以在后面可继续使用 x
    }

    { // 返回值与作用域

    }
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

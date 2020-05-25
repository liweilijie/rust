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
        let s1 = gives_ownership(); // gives_ownership 将返回值 移给s1

        let s2 = String::from("hello"); // s2 进入作用域

        let s3 = takes_and_gives_back(s2); // s2 被移动到
                                                        // takes_and_gives_back 中
                                                        // 它也将返回值给s3
        println!("s1:{}, s3:{}", s1, s3);
    } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
      // 所以什么也不会发生。s1 移出作用域并被丢弃

    { // 转移返回值的所有权
      // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。
      // 当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

    }
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String { // gives_ownership 将返回值移动给
                                 // 调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string  // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string // 返回 a_string 并移出给调用的函数
}

//
// 引用与借用
//
pub fn references_and_borrowing() {
    {
        let s1 = String::from("hello");

        // 这些 & 符号就是 引用，它们允许你使用值但不获取其所有权。
        // &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。
        // 因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。
        // 注意：与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*。
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}", s1, len);

        // 我们将获取引用作为函数参数称为 借用（borrowing）。
        // 正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。
    }

    { // 如果想修改引用的数据，则要用到可变引用。
        // 首先，必须将 s 改为 mut。
        // 然后必须创建一个可变引用 &mut s 和接受一个可变引用 some_string: &mut String。
        let mut s = String::from("hello");

        change(&mut s);

        // 可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。
        // 这个限制的好处是 Rust 可以在编译时就避免数据竞争。
        // 这些代码会失败：
        /*
        let mut s = String::from("hello");

        let r1 = &mut s;
        let r2 = &mut s;

        println!("{}, {}", r1, r2);
         */

        // 一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有：
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            r1.push_str(", r1 push.");
        } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
        let r2 = &mut s;
        r2.push_str(", r2 push.");


        // 类似的规则也存在于同时使用可变与不可变引用中。这些代码会导致一个错误：
        /*
        let mut s = String::from("hello");
        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        let r3 = &mut s; // 大问题
        println!("{}, {}, and {}", r1, r2, r3);
         */

        // 我们 也 不能在拥有不可变引用的同时拥有可变引用。
        // 不可变引用的用户可不希望在他们的眼皮底下值就被意外的改变了！
        // 然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。


        // 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
        // 例如，因为最后一次使用不可变引用在声明可变引用之前，所以如下代码是可以编译的：
        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用

        let r3 = &mut s; // 没问题
        r3.push_str(", world.");
        println!("{}", r3)
        // 不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束，
        // 这也是创建可变引用 r3 的地方。它们的作用域没有重叠，所以代码是可以编译的。
    }

    { // 悬垂引用： Dangling References
      // 当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
        // 下面的代码会出错：
        /*
        let reference_to_nothing = dangle();
         */
        let reference = no_dangle();
        println!("{}", reference);
    }
}

// 这些 & 符号就是 引用，它们允许你使用值但不获取其所有权。
fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
   s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生

fn change(some_string: &mut String) {
    some_string.push_str(", world.");
}

/*
// 悬垂引用
fn dangle() -> &String { // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
 */
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}//  这样就没有任何错误了。所有权被移动出去，所以没有值被释放。

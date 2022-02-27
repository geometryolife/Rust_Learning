// 【例7-16】使用 as 关键字将引入作用域的类型进行重命名
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --略--
}

fn function2() -> IoResult<()> {
    // --略--
}

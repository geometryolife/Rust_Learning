// 【例7-15】将两个拥有相同名称的类型引入作用域时需要使用它们的父模块
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --略--
}

fn function2() -> io::Result<()> {
    // --略--
}

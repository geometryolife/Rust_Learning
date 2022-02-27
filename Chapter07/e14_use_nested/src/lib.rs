// 一般形式
use std::cmp::Ordering;
use std::io,

// 【例7-18】指定嵌套的路径来将拥有共同路径前缀的条目引入作用域
use std::(cmp::Ordering, io);

// 【例7-19】两行使用了 use 的语句，其中一行是另一行的子路径
use std::io;
use std::io::Write;

// 【例7-20】将示例 7-19 中的路径合并至一行 use 语句中
use std::io::{self, Write};

// 使用通配符 *
use std::collections::*;

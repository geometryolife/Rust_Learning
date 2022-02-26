// 【例6-2】枚举 Message 的变体拥有不同数量和类型的内嵌数据
enum Message {
    Quit, // 没有关联任何结构体
    Move { x: i32, y: i32}, // 包含一个匿名结构体
    Write(String), // 包含一个 String
    ChangeColor(i32, i32, i32), // 包含三个 i32 值
}

// 使用结构体存储和上述变体一样的数据
struct QuitMessage; // 空结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

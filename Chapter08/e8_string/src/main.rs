fn main() {
    // 【例8-11】创建一个新的空字符串
    let s = String::new();

    println!("{:?}", s);
    // === Output ===
    // ""

    // 【例8-12】使用 to_string 方法基于字符串字面量创建 String
    let data = "initial contents";

    // 实现了 Display trait 的类型可以调用 to_string 方法
    let s = data.to_string();

    println!("{:?}", s);
    // === Output ===
    // "initial contents"

    // 这个方法同样也可以直接作用于字面量
    let s = "Initial Contents".to_string();
    println!("{:?}", s);
    // === Output ===
    // "Initial Contents"

    // 【8-13】使用 String::from 函数基于字符串字面量创建 String
    let s = String::from("initial contents");
    println!("{:?}", s);
    // === Output ===
    // "initial contents"

    // 【例8-14】存储在字符串中的不同语言的问候
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

// 情景：某个大厨需要修正一份错误的订单，并亲自将它送给外面的客户。
// 【示例7-8】使用 super 开头构建相对路径来调用函数
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

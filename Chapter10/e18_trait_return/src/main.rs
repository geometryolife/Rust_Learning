use e18_trait_return::Summary;
use e18_trait_return::NewsArticle;
use e18_trait_return::Tweet;

fn main() {
    returns_summarizable();
}

// returns_summarizable 返回了一个 Tweet，但调用者却无法知晓这一信息
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

// === Output ===
// error[E0308]: `if` and `else` have incompatible types
//   --> src/main.rs:21:9
//    |
// 11 |   /     if switch {
// 12 |   |         NewsArticle {
//    |  _|_________-
// 13 | | |             headline: String::from("Penguins win the Stanley Cup Championship!"),
// 14 | | |             location: String::from("Pittsburgh, PA, USA"),
// 15 | | |             author: String::from("Iceburgh"),
// ...  | |
// 18 | | |             ),
// 19 | | |         }
//    | |_|_________- expected because of this
// 20 |   |     } else {
// 21 | / |         Tweet {
// 22 | | |             username: String::from("horse_ebooks"),
// 23 | | |             content: String::from("of course, as you probably already know, people"),
// 24 | | |             reply: false,
// 25 | | |             retweet: false,
// 26 | | |         }
//    | |_|_________^ expected struct `NewsArticle`, found struct `Tweet`
// 27 |   |     }
//    |   |_____- `if` and `else` have incompatible types
//    |
// help: you could change the return type to be a boxed trait object
//    |
// 10 | fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
//    |                                          ^^^^^^^        ^
// help: if you change the return type to expect trait objects, box the returned expressions
//    |
// 12 |         Box::new(NewsArticle {
// 13 |             headline: String::from("Penguins win the Stanley Cup Championship!"),
// 14 |             location: String::from("Pittsburgh, PA, USA"),
// 15 |             author: String::from("Iceburgh"),
// 16 |             content: String::from(
// 17 |                 "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
//  ...

// error[E0061]: this function takes 1 argument but 0 arguments were supplied
//   --> src/main.rs:6:5
//    |
// 6  |     returns_summarizable();
//    |     ^^^^^^^^^^^^^^^^^^^^-- supplied 0 arguments
//    |     |
//    |     expected 1 argument
//    |
// note: function defined here
//   --> src/main.rs:10:4
//    |
// 10 | fn returns_summarizable(switch: bool) -> impl Summary {
//    |    ^^^^^^^^^^^^^^^^^^^^ ------------

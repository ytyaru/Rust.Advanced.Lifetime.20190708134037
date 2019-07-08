/*
 * Rustの高度な機能（ライフタイム）。
 * CreatedAt: 2019-07-08
 */
struct Context(&str);
struct Parser {
    context: &Context,
}
impl Parser {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}
fn main() {
    println!("Hello Rust !!");
}


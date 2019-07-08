/*
 * Rustの高度な機能（ライフタイム）。
 * CreatedAt: 2019-07-08
 */
fn main() {
    println!("Hello Rust !!");
}
struct Context<'a>(&'a str);
struct Parser<'a> {
    context: &'a Context<'a>,
}
impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}

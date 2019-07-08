/*
 * Rustの高度な機能（ライフタイム）。
 * CreatedAt: 2019-07-08
 */
fn main() {
    println!("Hello Rust !!");
}
struct Context<'s>(&'s str);
struct Parser<'c, 's> {
    context: &'c Context<'s>,
}
impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

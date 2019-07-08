/*
 * Rustの高度な機能（ライフタイム）。
 * CreatedAt: 2019-07-08
 */
trait Red { }
struct Ball<'a> {
    diameter: &'a i32,
}
impl<'a> Red for Ball<'a> { }
fn main() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<Red>;
}

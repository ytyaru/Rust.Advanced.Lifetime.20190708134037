/*
 * Rustの高度な機能（ライフタイム）。
 * CreatedAt: 2019-07-08
 */
fn main() {
    println!("Hello Rust !!");
}
//struct Ref<T>(&T); // error[E0106]: missing lifetime specifier
//struct Ref<'a, T>(&'a T);
struct Ref<'a, T: 'a>(&'a T);

struct StaticRef<T: 'static>(&'static T);

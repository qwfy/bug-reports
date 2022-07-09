use sqlx;

trait ColumnValue: for<'r> sqlx::Encode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> {
}

pub fn push_where_bad(cursors: Vec<Box<dyn ColumnValue>>)
{
}

// *********** The above code gave the following error ***********
// Search for string "LOOK HERE" in this file to see the bug

// error[E0038]: the trait `ColumnValue` cannot be made into an object
//    --> src/lib.rs:6:40
//     |
// 6   | pub fn push_where_bad(cursors: Vec<Box<dyn ColumnValue>>)
//     |                                        ^^^^^^^^^^^^^^^ `ColumnValue` cannot be made into an object
//     |
// note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
//    --> /home/incomplete/.cargo/registry/src/github.com-1ecc6299db9ec823/sqlx-core-0.5.13/src/types/mod.rs:175:8
//     |
// 175 |     fn type_info() -> DB::TypeInfo;
//     |        ^^^^^^^^^ ...because associated function `type_info` has no `self` parameter
// ...
// 184 |     fn compatible(ty: &DB::TypeInfo) -> bool {
//     |        ^^^^^^^^^^ ...because associated function `compatible` has no `self` parameter
//     |
//    ::: src/lib.rs:3:7
//     |
// 3   | trait ColumnValue: for<'r> sqlx::Encode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> {
//     |       ----------- this trait cannot be made into an object...
//     = help: consider turning `type_info` into a method by giving it a `&self` argument or constraining it so it does not apply to trait objects
//     = help: consider turning `compatible` into a method by giving it a `&self` argument or constraining it so it does not apply to trait objects
// help: consider turning `type_info` into a method by giving it a `&self` argument
//     |
// 175 |     fn type_info&self() -> DB::TypeInfo;
//     |                 +++++ <===== LOOK HERE
// help: consider turning `compatible` into a method by giving it a `&self` argument
//     |
// 184 |     fn compatible&self, (ty: &DB::TypeInfo) -> bool {
//     |                  ++++++
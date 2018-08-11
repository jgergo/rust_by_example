fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;    // f64
    let default_integer = 7;    // i32

    // A type can also be infered from context
    let mut infered_type = 12;  //the type is infered form another line's type annotation
    infered_type = 123123124235345i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable i32
    mutable = 21;

    // The type cannot be changed
    mutable = true;

    // Variables can be overwritten with shadowing
    let mutable = true;

}
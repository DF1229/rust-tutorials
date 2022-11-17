// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut x = 100;        // x is created
    let y = &mut x;         // y is a mutateable reference to x
    *y += 100;              // the value of x is mutated via y, and y is dereferenced
    let z = &mut x;         // z is a mutateable reference to x
    *z += 1000;             // the value of x is mutated via z, and z is dereferenced
    assert_eq!(x, 1200);    // x is now equal to 100 + 100 + 1000 = 1200
}

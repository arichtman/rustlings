// move_semantics1.rs
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec = Vec::<i32>::new();

    fill_vec(&mut vec);

    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);

    vec.push(88);

    println!("{} has length {} content `{:?}`", "vec", vec.len(), vec);
}

fn fill_vec(vec:&mut Vec<i32>){

    vec.push(22);
    vec.push(44);
    vec.push(66);

}

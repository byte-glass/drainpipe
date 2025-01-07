fn main() {
    println!("Doukipudonktan!");

    let mut v = vec![100, 32, 57];
//     for i in &v {
//         println!("{i}");
//     }

    for i in &mut v {
        *i += 5;
        println!("{i}");
    }

    println!("ok");
}

use std::io::stdin;
pub fn input() -> String {
    let mut s = String::new();
    let res = stdin().read_line(&mut s);
    match res{
        Err(_) => {s = String::from("n");},
        _ => {}
    };
    s
}

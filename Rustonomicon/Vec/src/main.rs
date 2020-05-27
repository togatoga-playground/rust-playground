use ::Vec::MyVec;

fn main() {
    let mut a = MyVec::new();
    for i in 0..1000 {
        a.push(i);
    }
    a.push("togatoga");
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

// TODO: add methods to Triangle
struct Triangle {
    l1: u32,
    l2: u32,
    l3: u32,
}

impl Triangle {
    fn print(t: &Self) {
        print!("l1: {}", t.l1);
        print!(", l2: {}", t.l2);
        print!(", l3: {}", t.l3);
        println!();
    }
    fn is_triangle(t: &Self) -> bool {
        // each l is < than the other 2
        return t.l1 < t.l2 + t.l3 && t.l2 < t.l1 + t.l3 && t.l3 < t.l1 + t.l2;
    }
}

fn main() {
    let t: Triangle = Triangle {
        l1: 2,
        l2: 2,
        l3: 3,
    };
    Triangle::print(&t);
    println!("{}", Triangle::is_triangle(&t));
}

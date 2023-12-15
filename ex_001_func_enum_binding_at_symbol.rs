#[derive(Debug)]
enum MyEnum {
    One,
    #[allow(dead_code)]
    Two,
}

fn func((x @ MyEnum::One | x @ MyEnum::Two): MyEnum) {
    println!("{:?}", x);
}

#[derive(Debug)]
enum MyEnum1 {
    #[allow(dead_code)]
    Direct(usize, usize),
    Reverse(usize, usize),
}

fn func1((MyEnum1::Direct(x, y) | MyEnum1::Reverse(y, x)): MyEnum1) {
    println!("x: {} | y: {}", x, y);
}

#[derive(Debug)]
struct Descriptor {
    name: String,
    note: Option<String>
}

fn func2((d @ Descriptor{note: (Some(note_content) | None), ..}): &Descriptor) {
}

fn main() {
    func(MyEnum::One);
    func1(MyEnum1::Reverse(1, 0));
}

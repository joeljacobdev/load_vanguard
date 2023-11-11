fn main() {
    let x = {
        let y = 1; // first statement
        let z = 2; // second statement
        y + z // this is the *tail* - what the whole block will evaluate to
    };
    println!("Hello, world! {x}");

    struct Number {
        _odd: bool,
        value: i32,
    }

    impl Number {
        fn is_positive(&self) -> bool {
            self.value > 0
        }
    }
    let mut a = Number {
        _odd: false,
        value: 4,
    };
    let Number { value, .. } = a;
    a.value = -1;
    println!("{} {}", value, a.is_positive());
    let result = match a.value {
        v if v > 0 => "Positive",
        _ => "Negative",
    };
    println!("{} ", result);
    let mut b = Number {
        _odd: false,
        value: 4,
    };

    fn invert(number: &mut Number) {
        number.value = -number.value;
    }
    // to borrow as mut, need to declare as mut
    invert(&mut b);
    println!("{}", b.value);

    fn print_type_name<T>(_var: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    struct Pair<T> {
        _a: T,
        _b: i32,
    }
    let a = Pair { _a: false, _b: 3 };
    print_type_name(&a);

    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(3);
    let v1 = vec![1, 2, 3];
    println!("{:?}", v1);
    let v1 = vec![1, 2, 5];
    println!("{:?}", v1);
    let v1 = vec![1, 2, 6];
    println!("{:?}", v1);
}

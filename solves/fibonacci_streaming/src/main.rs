use num_bigint::BigUint;

fn all_fibonacci_numbers() -> impl Iterator<Item = BigUint> {
    struct Fibonacci {
        last: BigUint,
        current: BigUint,
        started: bool,
    }

    impl Iterator for Fibonacci {
        type Item = BigUint;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.started {
                self.started = true;
                return Some(BigUint::from(1u32));
            }

            let next = &self.current + &self.last;

            self.last = std::mem::replace(&mut self.current, next);

            Some(self.current.clone())
        }
    }

    Fibonacci {
        last: BigUint::from(0u32),
        current: BigUint::from(1u32),
        started: false,
    }
}

fn main()
{
    for i in all_fibonacci_numbers().take(10) {
        println!("{}", i);
    }
}
use std::env;

fn main() {
    let arg = env::args().nth(1);

    let Some(limit) = arg else {
        return;
    };

    let limit = limit.parse().unwrap();
    let mut basket = vec![true; limit];

    for number in 2..limit {
        if basket[number] {
            let mut multiple = number * 2;

            while multiple < limit {
                basket[multiple] = false;
                multiple += number;
            }
        }
    }

    let mut out = Vec::new();

    for (num, &prime) in basket.iter().enumerate() {
        if prime {
            out.push(num);
        }
    }

    println!("{:?}", out);
}

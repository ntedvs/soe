use std::env;

fn main() {
    let arg = env::args().nth(1);

    let Some(limit) = arg else {
        return;
    };

    let limit = limit.parse().unwrap();
    let end = (limit as f64).sqrt() as usize + 1;

    let mut basket = vec![true; limit];

    for number in 2..end {
        if basket[number] {
            let mut multiple = number * number;

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

use rayon::join;
use rayon::prelude::*;
use std::thread::sleep;
use std::time::Duration;

pub fn square_split(v: &mut [i32]) {
    if v.len() < 4 {
        for i in v {
            sleep(Duration::from_millis(100));
            println!("{}", *i);
            *i *= *i;
        }
        return;
    }
    let (mut a, mut b) = v.split_at_mut(v.len() / 2);
    join(|| square_split(&mut a), || square_split(&mut b));
}

fn main() {
    let mut v = Vec::with_capacity(500);
    for i in 0..500 {
        v.push(i);
    }

    // square_split(&mut v);
    // v.par_iter_mut().for_each(|x|{
    //     sleep(Duration::from_millis(100));
    //     println!("{}",*x);
    //     *x *= *x
    // });

    let v2: Vec<i32> = (&v)
        .par_iter()
        .map(|&x| {
            sleep(Duration::from_millis(100));
            println!("{}", x);
            x * x
        })
        .collect();

    println!("{:?}", &v[490..500]);
    println!("{:?}", &v2[490..500]);
}

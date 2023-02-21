use rayon::prelude::*;

fn main()
{
    let start = std::time::Instant::now();
    const size:u64 = 10u64.pow(9);
    let s = size as u64;
    let s_cutoff = s*7/10;

    let in_square = s*s;
    let in_circle = 
    (0..s).into_par_iter()
        .map(|y|
        {
            let mut in_circle = 0;
            let mut record_x = 0;
            let mut start_x = 0;
            let mut delta = s/2;
            for i in 0..(s as f64).log2() as u64
            {
                if start_x * start_x + y * y < s*s
                {
                    record_x = start_x;
                    start_x += delta;
                }
                else
                {
                    start_x -= delta;
                }
                delta = (delta/2).max(1);
            }
            start_x = record_x;
            //binary search for circle edge complete
            for x in start_x..s
            {
                if x * x + y * y < s * s
                {
                    in_circle += 1;
                }
                else
                {
                    break;
                }
            }
            in_circle += start_x-1;
            in_circle
        }).reduce(|| 0, |a,b| a+b);

    let end = std::time::Instant::now();
    let duration = end - start;
    println!("Program took {} seconds to execute", duration.as_secs_f32());

    let approx_to_str = format!("{}", (in_circle as f64)/((in_square / 4) as f64));
    let pi_to_str = format!("{}", std::f64::consts::PI);
    println!("approximated pi:\t{}", approx_to_str);
    println!("actual pi:\t\t{}", pi_to_str);

    let matchcount = approx_to_str
        .chars()
        .zip(pi_to_str.chars())
        .filter(|(c,c2)| c==c2)
        .count() - 2;
    println!("the first {} digits are correct!", matchcount);

}


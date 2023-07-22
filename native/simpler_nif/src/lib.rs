use rustler::{NifMap};

#[derive(NifMap)]
#[derive(Debug)]
#[derive(PartialEq)]
struct ParamsMap {
    sum: i32,
    mean: f32,
    quartiles: Vec<f32>
}

#[rustler::nif(name = "get_params")]
fn get_params_nif(xs: Vec<i32>) -> ParamsMap {
    get_params(xs)
}

fn get_params(xs: Vec<i32>) -> ParamsMap {
    let s = xs.iter().sum();
    let (qs, len) = quartiles(xs);
    let m = s as f32 / len as f32;
    ParamsMap { sum: s, mean: m, quartiles: qs }
}

fn quartiles(mut xs: Vec<i32>) -> (Vec<f32>, usize) {
    let len = xs.len();
    xs.sort();
    let q2 = median_from_sorted(&xs, len);
    let sublen = len / 2;  // integer division
    let qs =
        match len % 2 {
            0 => {
                let q1 = median_from_sorted(&xs[0..sublen - 1], sublen);
                let q3 = median_from_sorted(&xs[sublen..], sublen);
                vec![q1, q2, q3]
            },
            _ => {
                let q1 = median_from_sorted(&xs[0..sublen], sublen + 1);
                let q3 = median_from_sorted(&xs[sublen..], sublen + 1);
                vec![q1, q2, q3]
            },
        };
    (qs, len)
}

fn median_from_sorted(xs: &[i32], len: usize) -> f32 {
    let a = xs[(len / 2) - 1];
    let b = xs[len / 2];
    if len % 2 == 1 {
        b as f32
    } else {
        (a + b) as f32 / 2 as f32
    }
}

rustler::init!("simpler_nif",
    [ get_params_nif
    ]
);

#[cfg(test)]
mod tests {
    use crate::get_params;
    use crate::ParamsMap;

    #[test]
    fn vec_in_length_even() {
        let expected = ParamsMap { sum: 36,
                                   mean: 4.5,
                                   quartiles: vec![2.5, 4.5, 6.5] };
        let actual = get_params(vec![1,3,5,7,8,6,4,2]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn vec_in_length_odd() {
        let expected = ParamsMap { sum: 45,
                                   mean: 5.0,
                                   quartiles: vec![3.0, 5.0, 7.0] };
        let actual = get_params(vec![9,1,3,5,7,8,6,4,2]);
        assert_eq!(actual, expected);
    }
}

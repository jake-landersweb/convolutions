use crate::padding;
use rand::Rng;

pub fn max_pool(input: &Vec<Vec<f64>>, size: usize) -> Vec<Vec<f64>> {
    return pool(input, size, |x| {
        *x.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    });
}

pub fn min_pool(input: &Vec<Vec<f64>>, size: usize) -> Vec<Vec<f64>> {
    return pool(input, size, |x| {
        *x.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    });
}

pub fn l2_pool(input: &Vec<Vec<f64>>, size: usize) -> Vec<Vec<f64>> {
    return pool(input, size, |x| x.iter().map(|v| v * v).sum::<f64>().sqrt());
}

pub fn stochastic_pooling(input: &Vec<Vec<f64>>, size: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    return pool(input, size, |x| x[rng.gen_range(0..x.len())]);
}

fn pool<P>(input: &Vec<Vec<f64>>, size: usize, mut predicate: P) -> Vec<Vec<f64>>
where
    P: FnMut(Vec<f64>) -> f64,
{
    // pad the input
    let mut padded_input = padding::zero_pad(input, size);
    // allocate space
    let mut out = vec![vec![0.; input[0].len()]; input.len()];

    // loop over the 2d range of the input
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            // inside loop body use fancy functions to pull 1d window
            let window = padded_input[i..i + size]
                .iter()
                .map(|x| x[j..j + size].iter().map(|n| *n).collect::<Vec<f64>>())
                .flatten()
                .collect::<Vec<f64>>();

            // call passed in function
            out[i][j] = predicate(window);
        }
    }

    return out;
}

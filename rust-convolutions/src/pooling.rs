use crate::padding;

fn pool<P>(input: &Vec<Vec<f64>>, size: usize, mut predicate: P) -> Vec<Vec<f64>>
where
    P: FnMut(Vec<f64>) -> f64,
{
    let mut padded_input = padding::zero_pad(input, size);
    let mut out = vec![vec![0.; input[0].len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let window = padded_input[i..i + size]
                .iter()
                .map(|x| x[j..j + size].iter().map(|n| *n).collect::<Vec<f64>>())
                .flatten()
                .collect::<Vec<f64>>();

            out[i][j] = predicate(window);
        }
    }

    return out;
}

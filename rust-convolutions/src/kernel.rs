pub type Kernel = Vec<Vec<f64>>;

pub trait KernelConstructors {
    fn identity() -> Self;
    fn blur() -> Self;
    fn gaussian(size: usize, sigma: f64) -> Self;
    fn edge_x() -> Self;
    fn edge_y() -> Self;
    fn edge_all() -> Self;
    fn sharpen() -> Self;
    fn emboss() -> Self;
    fn edge_enhance() -> Self;
}

impl KernelConstructors for Kernel {
    fn identity() -> Self {
        vec![vec![0., 0., 0.], vec![0., 1., 0.], vec![0., 0., 0.]]
    }
    fn blur() -> Self {
        vec![
            vec![1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
            vec![1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
            vec![1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0],
        ]
    }
    fn gaussian(size: usize, sigma: f64) -> Self {
        let center = (size / 2) as f64;
        let variance = sigma.powi(2);

        let mut kernel: Vec<Vec<f64>> = vec![vec![0.0; size]; size];

        for i in 0..size {
            for j in 0..size {
                let x = (i as f64) - center;
                let y = (j as f64) - center;

                let g = (1.0 / (2.0 * std::f64::consts::PI * variance))
                    * (-1.0 * ((x.powi(2) + y.powi(2)) / (2.0 * variance))).exp();
                kernel[i][j] = g;
            }
        }

        let sum: f64 = kernel.iter().flatten().sum();
        for i in 0..size {
            for j in 0..size {
                kernel[i][j] /= sum;
            }
        }

        kernel
    }
    fn edge_x() -> Self {
        vec![
            vec![-1.0, 0.0, 1.0],
            vec![-2.0, 0.0, 2.0],
            vec![-1.0, 0.0, 1.0],
        ]
    }
    fn edge_y() -> Self {
        vec![
            vec![-1.0, -2.0, -1.0],
            vec![0.0, 0.0, 0.0],
            vec![1.0, 2.0, 1.0],
        ]
    }
    fn edge_all() -> Self {
        vec![
            vec![0.0, 1.0, 0.0],
            vec![1.0, -4.0, 1.0],
            vec![0.0, 1.0, 0.0],
        ]
    }
    fn sharpen() -> Self {
        vec![vec![0., -1., 0.], vec![-1., 5., -1.], vec![0., -1., 0.]]
    }
    fn emboss() -> Self {
        vec![vec![-2., -1., 0.], vec![-1., 1., 1.], vec![0., 1., 2.]]
    }
    fn edge_enhance() -> Self {
        vec![
            vec![-1., -1., -1., -1., -1., -1., -1.],
            vec![-1., -1., -1., -1., -1., -1., -1.],
            vec![-1., -1., -1., -1., -1., -1., -1.],
            vec![-1., -1., -1., 49., -1., -1., -1.],
            vec![-1., -1., -1., -1., -1., -1., -1.],
            vec![-1., -1., -1., -1., -1., -1., -1.],
            vec![-1., -1., -1., -1., -1., -1., -1.],
        ]
    }
}

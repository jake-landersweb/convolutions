/// Pads the matrix using zero padding, based on the kernel size.
/// zero padding is where the vector is center-padded as such where
/// when a convolution occurs the output will be the same size as the
/// input.
pub fn zero_pad(input: &Vec<Vec<f64>>, size: usize) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = vec![vec![0.0; input[0].len() + size - 1]; input.len() + size - 1];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            out[i + size / 2][j + size / 2] = input[i][j];
        }
    }

    return out;
}

/// Pads a matrix using reflection, based on the kernel size.
/// reflection is where a matrix is center-padded with the reflection
/// of the previous row/column, where after a convolution occurs the
/// output will be the same size as the input.
///
/// For example:
/// input = [[1,2,3]
///          [4,5,6]
///          [7,8,9]]
/// kernel = (3x3)
///
/// output = [[5,4,5,6,5]
///           [2,1,2,3,2]
///           [5,4,5,6,5]
///           [8,7,8,9,8]
///           [5,4,5,6,5]
pub fn reflection_pad(input: &Vec<Vec<f64>>, size: usize) -> Vec<Vec<f64>> {
    let (input_rows, input_cols) = (input.len(), input[0].len());

    // calculate the amount of padding needed for each side
    let (pad_top, pad_bottom) = ((size - 1) / 2, size / 2);
    let (pad_left, pad_right) = ((size - 1) / 2, size / 2);

    // create the output matrix with the correct size
    let mut output =
        vec![vec![0.0; input_cols + pad_left + pad_right]; input_rows + pad_top + pad_bottom];

    // fill in the original input values
    for i in 0..input_rows {
        for j in 0..input_cols {
            output[i + pad_top][j + pad_left] = input[i][j];
        }
    }

    // reflect the top and bottom rows
    for i in 0..pad_top {
        let top_row = input[pad_top - i].clone();
        let bottom_row = input[input_rows - 1 - (pad_bottom - i)].clone();
        for j in 0..input_cols {
            output[i][j + pad_left] = top_row[j];
            let l = output.len();
            output[l - 1 - i][j + pad_left] = bottom_row[j];
        }
    }

    // reflect the left and right columns
    for j in 0..pad_left {
        let left_col = output
            .iter()
            .map(|row| row[pad_left - j + 1])
            .collect::<Vec<f64>>();
        let right_col = output
            .iter()
            .map(|row| row[input_cols - 1 + pad_left - (pad_right - j)])
            .collect::<Vec<f64>>();
        for i in 0..output.len() {
            output[i][j] = left_col[i];
            let l = output[0].len();
            output[i][l - 1 - j] = right_col[i];
        }
    }

    output
}

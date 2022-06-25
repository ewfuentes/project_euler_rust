extern crate nalgebra as na;

use itertools::Itertools;

const BLOCK_LENGTH: usize = 4;

fn compute_row_prod<R, C, S>(row: usize, col: usize, matrix: &na::Matrix<f64, R, C, S>) -> f64
where
    R: na::Dim,
    C: na::Dim,
    S: na::Storage<f64, R, C>,
{
    if col > matrix.ncols() - BLOCK_LENGTH {
        return 0.0;
    }
    return (0..BLOCK_LENGTH)
        .map(|step| *matrix.index((row, col + step)))
        .product();
}
fn compute_col_prod<R, C, S>(row: usize, col: usize, matrix: &na::Matrix<f64, R, C, S>) -> f64
where
    R: na::Dim,
    C: na::Dim,
    S: na::Storage<f64, R, C>,
{
    if row > matrix.nrows() - BLOCK_LENGTH {
        return 0.0;
    }
    return (0..BLOCK_LENGTH)
        .map(|step| *matrix.index((row+step, col)))
        .product();
}
fn compute_up_diag_prod<R, C, S>(row: usize, col: usize, matrix: &na::Matrix<f64, R, C, S>) -> f64
where
    R: na::Dim,
    C: na::Dim,
    S: na::Storage<f64, R, C>,
{

    if row < BLOCK_LENGTH - 1 || col > matrix.ncols() - BLOCK_LENGTH {
        return 0.0;
    }

    return (0..BLOCK_LENGTH)
        .map(|step| *matrix.index((row-step, col+step)))
        .product();
}
fn compute_down_diag_prod<R, C, S>(row: usize, col: usize, matrix: &na::Matrix<f64, R, C, S>) -> f64
where
    R: na::Dim,
    C: na::Dim,
    S: na::Storage<f64, R, C>,
{

    if row > matrix.nrows() - BLOCK_LENGTH || col > matrix.ncols() - BLOCK_LENGTH {
        return 0.0;
    }
    return (0..BLOCK_LENGTH)
        .map(|step| *matrix.index((row+step, col+step)))
        .product();
}

fn compute_max_prod<R, C, S>(row: usize, col: usize, matrix: &na::Matrix<f64, R, C, S>) -> f64
where
    R: na::Dim,
    C: na::Dim,
    S: na::Storage<f64, R, C>,
{
    let calcs = [
        compute_row_prod,
        compute_col_prod,
        compute_up_diag_prod,
        compute_down_diag_prod,
    ];

    return calcs
        .iter()
        .map(|x| x(row, col, &matrix))
        .reduce(f64::max)
        .unwrap();
}

fn main() {
    let matrix: na::SMatrix<f64, 20, 20> = na::matrix![
    08.0, 02.0, 22.0, 97.0, 38.0, 15.0, 00.0, 40.0, 00.0, 75.0, 04.0, 05.0, 07.0, 78.0, 52.0, 12.0, 50.0, 77.0, 91.0, 08.0;
    49.0, 49.0, 99.0, 40.0, 17.0, 81.0, 18.0, 57.0, 60.0, 87.0, 17.0, 40.0, 98.0, 43.0, 69.0, 48.0, 04.0, 56.0, 62.0, 00.0;
    81.0, 49.0, 31.0, 73.0, 55.0, 79.0, 14.0, 29.0, 93.0, 71.0, 40.0, 67.0, 53.0, 88.0, 30.0, 03.0, 49.0, 13.0, 36.0, 65.0;
    52.0, 70.0, 95.0, 23.0, 04.0, 60.0, 11.0, 42.0, 69.0, 24.0, 68.0, 56.0, 01.0, 32.0, 56.0, 71.0, 37.0, 02.0, 36.0, 91.0;
    22.0, 31.0, 16.0, 71.0, 51.0, 67.0, 63.0, 89.0, 41.0, 92.0, 36.0, 54.0, 22.0, 40.0, 40.0, 28.0, 66.0, 33.0, 13.0, 80.0;
    24.0, 47.0, 32.0, 60.0, 99.0, 03.0, 45.0, 02.0, 44.0, 75.0, 33.0, 53.0, 78.0, 36.0, 84.0, 20.0, 35.0, 17.0, 12.0, 50.0;
    32.0, 98.0, 81.0, 28.0, 64.0, 23.0, 67.0, 10.0, 26.0, 38.0, 40.0, 67.0, 59.0, 54.0, 70.0, 66.0, 18.0, 38.0, 64.0, 70.0;
    67.0, 26.0, 20.0, 68.0, 02.0, 62.0, 12.0, 20.0, 95.0, 63.0, 94.0, 39.0, 63.0, 08.0, 40.0, 91.0, 66.0, 49.0, 94.0, 21.0;
    24.0, 55.0, 58.0, 05.0, 66.0, 73.0, 99.0, 26.0, 97.0, 17.0, 78.0, 78.0, 96.0, 83.0, 14.0, 88.0, 34.0, 89.0, 63.0, 72.0;
    21.0, 36.0, 23.0, 09.0, 75.0, 00.0, 76.0, 44.0, 20.0, 45.0, 35.0, 14.0, 00.0, 61.0, 33.0, 97.0, 34.0, 31.0, 33.0, 95.0;
    78.0, 17.0, 53.0, 28.0, 22.0, 75.0, 31.0, 67.0, 15.0, 94.0, 03.0, 80.0, 04.0, 62.0, 16.0, 14.0, 09.0, 53.0, 56.0, 92.0;
    16.0, 39.0, 05.0, 42.0, 96.0, 35.0, 31.0, 47.0, 55.0, 58.0, 88.0, 24.0, 00.0, 17.0, 54.0, 24.0, 36.0, 29.0, 85.0, 57.0;
    86.0, 56.0, 00.0, 48.0, 35.0, 71.0, 89.0, 07.0, 05.0, 44.0, 44.0, 37.0, 44.0, 60.0, 21.0, 58.0, 51.0, 54.0, 17.0, 58.0;
    19.0, 80.0, 81.0, 68.0, 05.0, 94.0, 47.0, 69.0, 28.0, 73.0, 92.0, 13.0, 86.0, 52.0, 17.0, 77.0, 04.0, 89.0, 55.0, 40.0;
    04.0, 52.0, 08.0, 83.0, 97.0, 35.0, 99.0, 16.0, 07.0, 97.0, 57.0, 32.0, 16.0, 26.0, 26.0, 79.0, 33.0, 27.0, 98.0, 66.0;
    88.0, 36.0, 68.0, 87.0, 57.0, 62.0, 20.0, 72.0, 03.0, 46.0, 33.0, 67.0, 46.0, 55.0, 12.0, 32.0, 63.0, 93.0, 53.0, 69.0;
    04.0, 42.0, 16.0, 73.0, 38.0, 25.0, 39.0, 11.0, 24.0, 94.0, 72.0, 18.0, 08.0, 46.0, 29.0, 32.0, 40.0, 62.0, 76.0, 36.0;
    20.0, 69.0, 36.0, 41.0, 72.0, 30.0, 23.0, 88.0, 34.0, 62.0, 99.0, 69.0, 82.0, 67.0, 59.0, 85.0, 74.0, 04.0, 36.0, 16.0;
    20.0, 73.0, 35.0, 29.0, 78.0, 31.0, 90.0, 01.0, 74.0, 31.0, 49.0, 71.0, 48.0, 86.0, 81.0, 16.0, 23.0, 57.0, 05.0, 54.0;
    01.0, 70.0, 54.0, 71.0, 83.0, 51.0, 54.0, 69.0, 16.0, 92.0, 33.0, 48.0, 61.0, 43.0, 52.0, 01.0, 89.0, 19.0, 67.0, 48.0];

    let max_prod = (0..matrix.nrows())
        .cartesian_product(0..matrix.ncols())
        .map(|(x, y)| compute_max_prod(x, y, &matrix))
        .reduce(f64::max)
        .unwrap();
    println!("max_prod: {}", max_prod);
}

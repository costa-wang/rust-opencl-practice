use multiply::{ MultiplyKernel};

const RESULTS_TO_PRINT: usize = 20;

// Our arbitrary data set size (about a million) and coefficent:
const WORK_SIZE: usize = 1 << 20;
const COEFF: f32 = 5432.1;

fn main() {
    MultiplyKernel::create(WORK_SIZE);
}

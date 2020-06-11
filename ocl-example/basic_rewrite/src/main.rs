extern crate ocl_extras;
use multiply::{ MultiplyKernel};


const RESULTS_TO_PRINT: usize = 20;

// Our arbitrary data set size (about a million) and coefficent:
const WORK_SIZE: usize = 1 << 20;
const COEFF: f32 = 5432.1;

fn main() {
    let vec_source = ocl_extras::scrambled_vec((0.0, 20.0), WORK_SIZE);
    let mut kernel = MultiplyKernel::create(WORK_SIZE,&vec_source).unwrap();
    kernel.multiply(COEFF);

    let mut vec_result = vec![0.0f32; WORK_SIZE];
    kernel.result_buffer.read(&mut vec_result).enq();

    for idx in 0..WORK_SIZE {
        if idx < RESULTS_TO_PRINT {
            println!("source[{idx}]: {:.03}, \t coeff: {}, \tresult[{idx}]: {}",
            vec_source[idx], COEFF, vec_result[idx], idx = idx);
        }
        assert_eq!(vec_source[idx] * COEFF, vec_result[idx]);
    }
}

extern crate ocl_extras;
extern crate ocl;

use ocl::r#async::{BufferSink, WriteGuard};
use multiply::{ MultiplyKernel};


use std::thread::{JoinHandle, Builder as ThreadBuilder};

// Our arbitrary data set size (about a million) and coefficent:
const WORK_SIZE: usize = 1 << 20;
const COEFF: i32 = 321;

const THREAD_COUNT: usize = 32;

const RESULTS_TO_PRINT: usize = 20;


fn main() {
    let mut kernel = MultiplyKernel::create(WORK_SIZE).unwrap();
    let buffer_sink = kernel.buffer_sink;
    kernel.multiply(COEFF);

    let source_datas: Vec<_> = (0..THREAD_COUNT).map(|_| {
        ocl_extras::scrambled_vec((0, 20),WORK_SIZE)
    }).collect();
    let mut threads = Vec::<JoinHandle<()>>::with_capacity(THREAD_COUNT * 2);

    for i in 0..THREAD_COUNT {
        let writer_0 = buffer_sink.clone().write();
        threads.push(ThreadBuilder::new().name(format!("thread_{}", i)).spawn(move || {
            let mut write_guard = writer_0.wait().unwrap();
            write_guard.copy_from_slice(&[0i32; WORK_SIZE]);
            let buffer_sink: BufferSink<_> = WriteGuard::release(write_guard).into();
            buffer_sink.flush().enq().unwrap().await;
        }).unwrap());

        let source_data = source_datas[i].clone();

        let writer_1 = buffer_sink.clone().write();
        threads.push(ThreadBuilder::new().name(format!("thread_{}", i)).spawn(move || {
            let mut write_guard = writer_1.wait().unwrap();
            write_guard.copy_from_slice(&source_data);
            let buffer_sink: BufferSink<_> = WriteGuard::release(write_guard).into();
            buffer_sink.flush().enq().unwrap().await;
        }).unwrap());
        // // Check results:
        // for (&src, &res) in source_data.iter().zip(vec_result.iter()) {
        //     assert_eq!(src * COEFF, res);
        // }
    }
}

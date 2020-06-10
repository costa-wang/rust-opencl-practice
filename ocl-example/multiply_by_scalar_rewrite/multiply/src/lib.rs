extern crate ocl;
extern crate ocl_extras;
extern crate log;

use std::result::Result;
use ocl::{Buffer, MemFlags, ProQue,Error};
use log::info;

static MULTIPLY_SRC: &str = include_str!("kernel/multiply.cl");

// pub fn hello(){
//     println!("{}",MULTIPLY_SRC);
// }

pub struct MultiplyKernel
{
    proque: ProQue,
    source_buffer: Buffer<f32>,
    result_buffer: Buffer<f32>,
}

impl MultiplyKernel
{
    pub fn create(work_size: usize) -> Result<MultiplyKernel,Error> {
        // let devices = &GPU_NVIDIA_DEVICES;
        // if devices.is_empty() {
        //     return Err(GPUError::Simple("No working GPUs found!"));
        // }
        // let device = devices[0]; // Select the first device for FFT
        let pq = ProQue::builder().src(MULTIPLY_SRC).dims(work_size).build()?;


        let vec_source = ocl_extras::scrambled_vec((0.0, 20.0), pq.dims().to_len());
        
        let source_buffer = Buffer::builder()
        .queue(pq.queue().clone())
        .flags(MemFlags::new().read_write())
        .len(work_size)
        .copy_host_slice(&vec_source)
        .build()?;

        // let mut vec_result = vec![0.0f32; work_size];
        let result_buffer: Buffer<f32> = pq.create_buffer()?;

        info!("FFT: 1 working device(s) selected.");
        info!("FFT: Device 0: {}", pq.device().name()?);

        Ok(MultiplyKernel {
            proque: pq,
            source_buffer: source_buffer,
            result_buffer: result_buffer,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

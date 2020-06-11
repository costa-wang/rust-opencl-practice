extern crate ocl;
extern crate log;

use std::result::Result;
use ocl::{Buffer, MemFlags, ProQue,Error};
use log::info;

static MULTIPLY_SRC: &str = include_str!("kernel/multiply.cl");

pub struct MultiplyKernel
{
    proque: ProQue,
    source_buffer: Buffer<f32>,
    pub result_buffer: Buffer<f32>
}

impl MultiplyKernel
{
    pub fn create(work_size: usize,vec_source: &Vec<f32>) -> Result<MultiplyKernel,Error> {
        // let devices = &GPU_NVIDIA_DEVICES;
        // if devices.is_empty() {
        //     return Err(GPUError::Simple("No working GPUs found!"));
        // }
        // let device = devices[0]; // Select the first device for FFT
        let pq = ProQue::builder().src(MULTIPLY_SRC).dims(work_size).build()?;

        
        let source_buffer = Buffer::builder()
        .queue(pq.queue().clone())
        .flags(MemFlags::new().read_write())
        .len(work_size)
        .copy_host_slice(vec_source)
        .build()?;

        let result_buffer: Buffer<f32> = pq.create_buffer()?;

        info!("FFT: 1 working device(s) selected.");
        info!("FFT: Device 0: {}", pq.device().name()?);

        Ok(MultiplyKernel {
            proque: pq,
            source_buffer: source_buffer,
            result_buffer: result_buffer,
        })
    }

    pub fn multiply(&mut self,work_size: usize,coeff:f32){
        let kern = self.proque.kernel_builder("multiply_by_scalar")
        .arg(coeff)
        .arg(None::<&Buffer<f32>>)
        .arg_named("result", None::<&Buffer<f32>>)
        .build().unwrap();
        
        // Set our named argument. The Option<_> wrapper is, well... optional:
        kern.set_arg("result", &self.result_buffer);
        // We can also set arguments (named or not) by index. Just for
        // demonstration, we'll set one using an option:
        kern.set_arg(0, &coeff);
        kern.set_arg(1, Some(&self.source_buffer));
        kern.set_arg(2, &self.result_buffer);
        
        unsafe { kern.enq();}
        // Read results from the device into result_buffer's local vector:
    }
}


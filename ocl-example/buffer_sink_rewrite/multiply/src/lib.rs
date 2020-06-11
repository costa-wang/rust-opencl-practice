extern crate ocl;
extern crate log;

use std::result::Result;
use ocl::{Buffer, MemFlags, ProQue,Error};
use ocl::r#async::{BufferSink, WriteGuard};
use log::info;

static MULTIPLY_SRC: &str = include_str!("kernel/multiply.cl");

pub struct MultiplyKernel
{
    proque: ProQue,
    source_buffer: Buffer<i32>,
    pub result_buffer: Buffer<i32>,
    pub buffer_sink: BufferSink<i32>
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

        
        let source_buffer = Buffer::builder()
        .queue(pq.queue().clone())
        .flags(MemFlags::new().read_write())
        .len(work_size)
        .build()?;

        let result_buffer: Buffer<i32> = pq.create_buffer()?;

        info!("FFT: 1 working device(s) selected.");
        info!("FFT: Device 0: {}", pq.device().name()?);

        let buffer_sink = unsafe {
            BufferSink::from_buffer(source_buffer.clone(), Some(pq.queue().clone()), 0,
            work_size)?
        };

        Ok(MultiplyKernel {
            proque: pq,
            source_buffer: source_buffer,
            result_buffer: result_buffer,
            buffer_sink: buffer_sink
        })
    }

    pub fn multiply(&mut self,coeff: i32){

        let kern = self.proque.kernel_builder("multiply_by_scalar")
        .arg(coeff)
        .arg(&self.source_buffer)
        .arg(&self.result_buffer)
        .build().unwrap();
        
        unsafe { kern.enq();}
        // Read results from the device into result_buffer's local vector:
    }
}


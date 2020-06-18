//! Print information about all the things.
//!
//! Printing info for any of the main types is as simple as
//! `println("{}", &instance);` as `Display` is implemented for each.
//!
//! Printing algorithm is highly janky (due to laziness -- need to complete
//! for each `*InfoResult` type) so lots of stuff isn't formatted correctly
//! (or at all).
//!
//!

extern crate ocl;
#[macro_use] extern crate colorify;

use ocl::{Result as OclResult, Platform, Device, Context, Queue, Buffer, Image, Sampler, Program,
    Kernel, Event, EventList};
use ocl::core::{ProgramInfo, OclPrm};

const PRINT_DETAILED: bool = true;
// Overrides above for device and program:
const PRINT_DETAILED_DEVICE: bool = false;
const PRINT_DETAILED_PROGRAM: bool = false;

static TAB: &'static str = "    ";
static SRC: &'static str = r#"
    __kernel void multiply(__global float* buffer, float coeff) {
        buffer[get_global_id(0)] *= coeff;
    }
"#;

 fn info() -> OclResult<()> {
    let dims = 2048;
    // Loop through all avaliable platforms:

    for (_p_idx, platform) in Platform::list().into_iter().enumerate() {
        print_platform_info(&platform);
        for (_d_idx, device) in Device::list_all(&platform)?.into_iter().enumerate() {
            let context = Context::builder().platform(platform).devices(device).build()?;
            let queue = Queue::new(&context, device, Some(ocl::core::QUEUE_PROFILING_ENABLE))?;
            let buffer = Buffer::<f32>::builder()
                .queue(queue.clone())
                .len(dims)
                .build()?;
            let image = Image::<u8>::builder()
                .dims(dims)
                .queue(queue.clone())
                .build()?;
            let sampler = Sampler::with_defaults(&context)?;
            let program = Program::builder()
                .src(SRC)
                .devices(device)
                .build(&context)?;
            let kernel = Kernel::builder()
                .name("multiply")
                .program(&program)
                .queue(queue.clone())
                .global_work_size(dims)
                .arg(&buffer)
                .arg(10.0f32)
                .build()?;

            let mut event_list = EventList::new();
            unsafe { kernel.cmd().enew(&mut event_list).enq()?; }
            event_list.wait_for()?;

            let mut event = Event::empty();
            buffer.cmd().write(&vec![0.0; dims]).enew(&mut event).enq()?;
            event.wait_for()?;

            // Print all but device (just once per platform):
                print_device_info(&device)?;
                print_context_info(&context);
                print_queue_info(&queue);
                print_buffer_info(&buffer);
                print_image_info(&image);
                print_sampler_info(&sampler);
                print_program_info(&program)?;
                print_kernel_info(&kernel);
                print_event_list_info(&event_list);
                print_event_info(&event);

        }
    }
    Ok(())
}


fn print_platform_info(platform: &Platform) -> OclResult<()> {
    printc!(blue: "{}", platform);
    let devices = Device::list_all(platform)?;
    printc!(blue: " {{ Total Device Count: {} }}", devices.len());
    print!("\n");
    Ok(())
}


fn print_device_info(device: &Device) -> OclResult<()> {
    if PRINT_DETAILED_DEVICE {
        printlnc!(teal: "{}", device);
    } else {
        if !PRINT_DETAILED { print!("{t}", t = TAB); }
        printlnc!(teal: "Device (terse) {{ Name: {}, Vendor: {} }}", device.name()?,
            device.vendor()?);
    }
    Ok(())
}


fn print_context_info(context: &Context) {
    printlnc!(purple: "{}", context);
}


fn print_queue_info(queue: &Queue) {
    printlnc!(lime: "{}", queue);
}


fn print_buffer_info<T: OclPrm>(buffer: &Buffer<T>) {
    printlnc!(royal_blue: "{}", buffer);
}


fn print_image_info<S: OclPrm>(image: &Image<S>) {
    printlnc!(peach: "{}", image);
}


fn print_sampler_info(sampler: &Sampler) {
    printlnc!(cyan: "{}", sampler);
}


fn print_program_info(program: &Program) -> OclResult<()> {
    if PRINT_DETAILED_PROGRAM {
        printlnc!(magenta: "{}", program);
    } else {
        if !PRINT_DETAILED { print!("{t}{t}", t = TAB); }
        printlnc!(magenta: "Program (terse) {{ KernelNames: '{}', NumDevices: {}, ReferenceCount: {}, Context: {} }}",
            program.info(ProgramInfo::KernelNames)?,
            program.info(ProgramInfo::NumDevices)?,
            program.info(ProgramInfo::ReferenceCount)?,
            program.info(ProgramInfo::Context)?,
        );
    }
    Ok(())
}


fn print_kernel_info(kernel: &Kernel) {
    printlnc!(green: "{}", kernel);
}


fn print_event_list_info(event_list: &EventList) {
    printlnc!(orange: "{:?}", event_list);
}


fn print_event_info(event: &Event) {
    printlnc!(yellow: "{}", event);
}


pub fn main() {
    match info(){
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}

// Platform { 
//     Profile: Ok(FULL_PROFILE), 
//     Version: Ok(OpenCL 2.0), 
//     Name: Ok(Intel(R) OpenCL), 
//     Vendor: Ok(Intel(R) Corporation), 
//     Extensions: Ok(
//         cl_khr_icd 
//         cl_khr_global_int32_base_atomics 
//         cl_khr_global_int32_extended_atomics 
//         cl_khr_local_int32_base_atomics 
//         cl_khr_local_int32_extended_atomics 
//         cl_khr_byte_addressable_store 
//         cl_khr_depth_images 
//         cl_khr_3d_image_writes 
//         cl_intel_exec_by_local_thread 
//         cl_khr_spir cl_khr_fp64 
//         cl_khr_image2d_from_buffer) 
// } 
// { Total Device Count: 1 }
// Device (terse) { 
//     Name: Intel(R) Core(TM) i7-6500U CPU @ 2.50GHz, 
//     Vendor: Intel(R) Corporation 
// }
// Context { 
//     ReferenceCount: Ok(1), 
//     Devices: Ok([DeviceId(0x5584ae671898)]), 
//     Properties: Ok(ContextProperties { 
//         props: {Platform: Platform(PlatformId(0x5584ae658b50))}, 
//         contains_gl_context_or_sharegroup: false }), 
//     NumDevices: Ok(1) 
// }
// Queue { 
//     Context: Ok(Context(0x5584ae64f138)), 
//     Device: Ok(DeviceId(0x5584ae671898)), 
//     ReferenceCount: Ok(4), 
//     Properties: Ok(PROFILING_ENABLE) 
// }
// Buffer Mem { 
//     Type: Ok(Buffer), 
//     Flags: Ok(READ_WRITE), 
//     Size: Ok(8192), 
//     HostPtr: Ok(None), 
//     MapCount: Ok(0), 
//     ReferenceCount: Ok(2), 
//     Context: Ok(Context(0x5584ae64f138)), 
//     AssociatedMemobject: Ok(None), 
//     Offset: Ok(0) 
// }
// Image { 
//     ElementSize: Ok(4), 
//     RowPitch: Ok(8192), 
//     SlicePitch: Ok(0), 
//     Width: Ok(2048), 
//     Height: Ok(0), 
//     Depth: Ok(0), 
//     ArraySize: Ok(0), 
//     Buffer: Ok(None), 
//     NumMipLevels: Ok(0), 
//     NumSamples: Ok(0) 
// } 
// Mem { 
//     Type: Ok(Image1d), 
//     Flags: Ok(READ_WRITE), 
//     Size: Ok(8192), 
//     HostPtr: Ok(None), 
//     MapCount: Ok(0), 
//     ReferenceCount: Ok(1), 
//     Context: Ok(Context(0x5584ae64f138)), 
//     AssociatedMemobject: Ok(None), 
//     Offset: Ok(0) 
// }
// Sampler { 
//     ReferenceCount: Ok(1), 
//     Context: Ok(Context(0x5584ae64f138)), 
//     NormalizedCoords: Ok(false), 
//     AddressingMode: Ok(None), 
//     FilterMode: Ok(Nearest) 
// }
// Program (terse) { 
//     KernelNames: 'multiply', 
//     NumDevices: 1, 
//     ReferenceCount: 1, 
//     Context: Context(0x5584ae64f138) 
// }
// Kernel { 
//     FunctionName: Ok(multiply), 
//     ReferenceCount: Ok(1), 
//     Context: Ok(Context(0x5584ae64f138)), 
//     Program: Ok(Program(0x5584af60fa68)), 
//     Attributes: Ok() } 
// WorkGroup { 
//     WorkGroupSize: Err(Kernel work-group info unavailable), 
//     CompileWorkGroupSize: Err(Kernel work-group info unavailable), 
//     LocalMemSize: Err(Kernel work-group info unavailable), 
//     PreferredWorkGroupSizeMultiple: Err(Kernel work-group info unavailable), 
//     PrivateMemSize: Err(Kernel work-group info unavailable) 
// }
// EventList { 
//     inner: Array(EventArray { array: [
//         Event(Event(0x5584af6673b8)), 
//         Event(Event(0x0)), 
//         Event(Event(0x0)), 
//         Event(Event(0x0)), 
//         Event(Event(0x0)), 
//         Event(Event(0x0)), 
//         Event(Event(0x0)), 
//         Event(Event(0x0))], 
//         len: 1 }) 
//     }
// Event { 
//     CommandQueue: Ok(CommandQueue(0x5584ae6732d8)), 
//     CommandType: Ok(WriteBuffer), ReferenceCount: Ok(1), 
//     CommandExecutionStatus: Ok(Complete), 
//     Context: Ok(Context(0x5584ae64f138)) 
// }

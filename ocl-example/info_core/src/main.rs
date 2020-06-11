//! Get information about all the things using `core` function calls.
//!
//! Set `INFO_FORMAT_MULTILINE` to `false` for compact printing.

extern crate ocl;

use ocl::core::{self, PlatformInfo, DeviceInfo, ContextInfo,
    CommandQueueInfo, MemInfo, ImageInfo, SamplerInfo, ProgramInfo,
    ProgramBuildInfo, KernelInfo, KernelArgInfo, KernelWorkGroupInfo,
    EventInfo, ProfilingInfo, Status};
use ocl::{Platform, Device, Context, Queue, Buffer, Image, Sampler, Program,
    Kernel, Event, EventList, SpatialDims};

const WORK_SIZE: [usize; 3] = [1024, 64, 16];
const INFO_FORMAT_MULTILINE: bool = true;

static SRC: &'static str = r#"
    __kernel void multiply(float coeff, __global float* buffer) {
        buffer[get_global_id(0)] *= coeff;
    }
"#;

/// Convert the info or error to a string for printing:
macro_rules! to_string {
    ( $ expr : expr ) => {
        match $expr {
            Ok(info) => info.to_string(),
            Err(err) => {
                match err.api_status() {
                    Some(Status::CL_KERNEL_ARG_INFO_NOT_AVAILABLE) => "Not available".into(),
                    _ => err.to_string(),
                }
            },
        }
    };
}


fn print_platform_device(plat_idx: usize, platform: Platform, device_idx: usize,
        device: Device) -> ocl::Result<()> {
    let work_dims = SpatialDims::from(WORK_SIZE);

    let context = Context::builder().platform(platform).devices(device).build()?;
    let program = Program::builder()
        .devices(device)
        .src(SRC)
        .build(&context)?;
    let queue = Queue::new(&context, device, Some(core::QUEUE_PROFILING_ENABLE))?;
    let buffer = Buffer::<f32>::builder()
        .queue(queue.clone())
        .len(work_dims)
        .build()?;
    let image = Image::<u8>::builder()
        .dims(work_dims)
        .queue(queue.clone())
        .build()?;
    let sampler = Sampler::with_defaults(&context)?;
        let kernel = Kernel::builder()
        .name("multiply")
        .program(&program)
        .queue(queue.clone())
        .global_work_size(work_dims)
        .arg(10.0f32)
        .arg(&buffer)
        .build()?;

    let mut event_list = EventList::new();
    unsafe { kernel.cmd().enew(&mut event_list).enq()?; }
    event_list.wait_for()?;

    let mut event = Event::empty();
    buffer.cmd().write(&vec![0.0; work_dims.to_len()]).enew(&mut event).enq()?;
    event.wait_for()?;

    let device_version = device.version()?;

    println!("############### OpenCL Platform-Device Full Info ################");
    print!("\n");

    let (begin, delim, end) = if INFO_FORMAT_MULTILINE {
        ("\n", "\n", "\n")
    } else {
        ("{ ", ", ", " }")
    };

    // ##################################################
    // #################### PLATFORM ####################
    // ##################################################

    println!("Platform [{}]:{b}\
            Profile: {}{d}\
            Version: {}{d}\
            Name: {}{d}\
            Vendor: {}{d}\
            Extensions: {}{e}\
        ",
        plat_idx,
        to_string!(core::get_platform_info(platform, PlatformInfo::Profile)),
        to_string!(core::get_platform_info(platform, PlatformInfo::Version)),
        to_string!(core::get_platform_info(platform, PlatformInfo::Name)),
        to_string!(core::get_platform_info(platform, PlatformInfo::Vendor)),
        to_string!(core::get_platform_info(platform, PlatformInfo::Extensions)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // #################### DEVICES #####################
    // ##################################################

    debug_assert!(context.devices().len() == 1);

    println!("Device [{}]: {b}\
            Type: {}{d}\
            VendorId: {}{d}\
            MaxComputeUnits: {}{d}\
            MaxWorkItemDimensions: {}{d}\
            MaxWorkGroupSize: {}{d}\
            MaxWorkItemSizes: {}{d}\
            PreferredVectorWidthChar: {}{d}\
            PreferredVectorWidthShort: {}{d}\
            PreferredVectorWidthInt: {}{d}\
            PreferredVectorWidthLong: {}{d}\
            PreferredVectorWidthFloat: {}{d}\
            PreferredVectorWidthDouble: {}{d}\
            MaxClockFrequency: {}{d}\
            AddressBits: {}{d}\
            MaxReadImageArgs: {}{d}\
            MaxWriteImageArgs: {}{d}\
            MaxMemAllocSize: {}{d}\
            Image2dMaxWidth: {}{d}\
            Image2dMaxHeight: {}{d}\
            Image3dMaxWidth: {}{d}\
            Image3dMaxHeight: {}{d}\
            Image3dMaxDepth: {}{d}\
            ImageSupport: {}{d}\
            MaxParameterSize: {}{d}\
            MaxSamplers: {}{d}\
            MemBaseAddrAlign: {}{d}\
            MinDataTypeAlignSize: {}{d}\
            SingleFpConfig: {}{d}\
            GlobalMemCacheType: {}{d}\
            GlobalMemCachelineSize: {}{d}\
            GlobalMemCacheSize: {}{d}\
            GlobalMemSize: {}{d}\
            MaxConstantBufferSize: {}{d}\
            MaxConstantArgs: {}{d}\
            LocalMemType: {}{d}\
            LocalMemSize: {}{d}\
            ErrorCorrectionSupport: {}{d}\
            ProfilingTimerResolution: {}{d}\
            EndianLittle: {}{d}\
            Available: {}{d}\
            CompilerAvailable: {}{d}\
            ExecutionCapabilities: {}{d}\
            QueueProperties: {}{d}\
            Name: {}{d}\
            Vendor: {}{d}\
            DriverVersion: {}{d}\
            Profile: {}{d}\
            Version: {}{d}\
            Extensions: {}{d}\
            Platform: {}{d}\
            DoubleFpConfig: {}{d}\
            HalfFpConfig: {}{d}\
            PreferredVectorWidthHalf: {}{d}\
            HostUnifiedMemory: {}{d}\
            NativeVectorWidthChar: {}{d}\
            NativeVectorWidthShort: {}{d}\
            NativeVectorWidthInt: {}{d}\
            NativeVectorWidthLong: {}{d}\
            NativeVectorWidthFloat: {}{d}\
            NativeVectorWidthDouble: {}{d}\
            NativeVectorWidthHalf: {}{d}\
            OpenclCVersion: {}{d}\
            LinkerAvailable: {}{d}\
            BuiltInKernels: {}{d}\
            ImageMaxBufferSize: {}{d}\
            ImageMaxArraySize: {}{d}\
            ParentDevice: {}{d}\
            PartitionMaxSubDevices: {}{d}\
            PartitionProperties: {}{d}\
            PartitionAffinityDomain: {}{d}\
            PartitionType: {}{d}\
            ReferenceCount: {}{d}\
            PreferredInteropUserSync: {}{d}\
            PrintfBufferSize: {}{d}\
            ImagePitchAlignment: {}{d}\
            ImageBaseAddressAlignment: {}{e}\
        ",
        device_idx,
        to_string!(core::get_device_info(&device, DeviceInfo::Type)),
        to_string!(core::get_device_info(&device, DeviceInfo::VendorId)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxComputeUnits)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxWorkItemDimensions)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxWorkGroupSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxWorkItemSizes)),
        to_string!(core::get_device_info(&device, DeviceInfo::PreferredVectorWidthChar)),
        to_string!(core::get_device_info(&device, DeviceInfo::PreferredVectorWidthShort)),
        to_string!(core::get_device_info(&device, DeviceInfo::PreferredVectorWidthInt)),
        to_string!(core::get_device_info(&device, DeviceInfo::PreferredVectorWidthLong)),
        to_string!(core::get_device_info(&device, DeviceInfo::PreferredVectorWidthFloat)),
        to_string!(core::get_device_info(&device, DeviceInfo::PreferredVectorWidthDouble)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxClockFrequency)),
        to_string!(core::get_device_info(&device, DeviceInfo::AddressBits)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxReadImageArgs)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxWriteImageArgs)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxMemAllocSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::Image2dMaxWidth)),
        to_string!(core::get_device_info(&device, DeviceInfo::Image2dMaxHeight)),
        to_string!(core::get_device_info(&device, DeviceInfo::Image3dMaxWidth)),
        to_string!(core::get_device_info(&device, DeviceInfo::Image3dMaxHeight)),
        to_string!(core::get_device_info(&device, DeviceInfo::Image3dMaxDepth)),
        to_string!(core::get_device_info(&device, DeviceInfo::ImageSupport)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxParameterSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxSamplers)),
        to_string!(core::get_device_info(&device, DeviceInfo::MemBaseAddrAlign)),
        to_string!(core::get_device_info(&device, DeviceInfo::MinDataTypeAlignSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::SingleFpConfig)),
        to_string!(core::get_device_info(&device, DeviceInfo::GlobalMemCacheType)),
        to_string!(core::get_device_info(&device, DeviceInfo::GlobalMemCachelineSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::GlobalMemCacheSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::GlobalMemSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxConstantBufferSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::MaxConstantArgs)),
        to_string!(core::get_device_info(&device, DeviceInfo::LocalMemType)),
        to_string!(core::get_device_info(&device, DeviceInfo::LocalMemSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::ErrorCorrectionSupport)),
        to_string!(core::get_device_info(&device, DeviceInfo::ProfilingTimerResolution)),
        to_string!(core::get_device_info(&device, DeviceInfo::EndianLittle)),
        to_string!(core::get_device_info(&device, DeviceInfo::Available)),
        to_string!(core::get_device_info(&device, DeviceInfo::CompilerAvailable)),
        to_string!(core::get_device_info(&device, DeviceInfo::ExecutionCapabilities)),
        to_string!(core::get_device_info(&device, DeviceInfo::QueueProperties)),
        to_string!(core::get_device_info(&device, DeviceInfo::Name)),
        to_string!(core::get_device_info(&device, DeviceInfo::Vendor)),
        to_string!(core::get_device_info(&device, DeviceInfo::DriverVersion)),
        to_string!(core::get_device_info(&device, DeviceInfo::Profile)),
        to_string!(core::get_device_info(&device, DeviceInfo::Version)),
        to_string!(core::get_device_info(&device, DeviceInfo::Extensions)),
        to_string!(core::get_device_info(&device, DeviceInfo::Platform)),
        to_string!(core::get_device_info(&device, DeviceInfo::DoubleFpConfig)),
        to_string!(core::get_device_info(&device, DeviceInfo::HalfFpConfig)),
        to_string!(core::get_device_info(&device, DeviceInfo::PreferredVectorWidthHalf)),
        to_string!(core::get_device_info(&device, DeviceInfo::HostUnifiedMemory)),
        to_string!(core::get_device_info(&device, DeviceInfo::NativeVectorWidthChar)),
        to_string!(core::get_device_info(&device, DeviceInfo::NativeVectorWidthShort)),
        to_string!(core::get_device_info(&device, DeviceInfo::NativeVectorWidthInt)),
        to_string!(core::get_device_info(&device, DeviceInfo::NativeVectorWidthLong)),
        to_string!(core::get_device_info(&device, DeviceInfo::NativeVectorWidthFloat)),
        to_string!(core::get_device_info(&device, DeviceInfo::NativeVectorWidthDouble)),
        to_string!(core::get_device_info(&device, DeviceInfo::NativeVectorWidthHalf)),
        to_string!(core::get_device_info(&device, DeviceInfo::OpenclCVersion)),
        to_string!(core::get_device_info(&device, DeviceInfo::LinkerAvailable)),
        to_string!(core::get_device_info(&device, DeviceInfo::BuiltInKernels)),
        to_string!(core::get_device_info(&device, DeviceInfo::ImageMaxBufferSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::ImageMaxArraySize)),
        to_string!(core::get_device_info(&device, DeviceInfo::ParentDevice)),
        to_string!(core::get_device_info(&device, DeviceInfo::PartitionMaxSubDevices)),
        to_string!(core::get_device_info(&device, DeviceInfo::PartitionProperties)),
        to_string!(core::get_device_info(&device, DeviceInfo::PartitionAffinityDomain)),
        to_string!(core::get_device_info(&device, DeviceInfo::PartitionType)),
        to_string!(core::get_device_info(&device, DeviceInfo::ReferenceCount)),
        to_string!(core::get_device_info(&device, DeviceInfo::PreferredInteropUserSync)),
        to_string!(core::get_device_info(&device, DeviceInfo::PrintfBufferSize)),
        to_string!(core::get_device_info(&device, DeviceInfo::ImagePitchAlignment)),
        to_string!(core::get_device_info(&device, DeviceInfo::ImageBaseAddressAlignment)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // #################### CONTEXT #####################
    // ##################################################

    println!("Context:{b}\
            Reference Count: {}{d}\
            Devices: {}{d}\
            Properties: {}{d}\
            Device Count: {}{e}\
        ",
        to_string!(core::get_context_info(&context, ContextInfo::ReferenceCount)),
        to_string!(core::get_context_info(&context, ContextInfo::Devices)),
        to_string!(core::get_context_info(&context, ContextInfo::Properties)),
        to_string!(core::get_context_info(&context, ContextInfo::NumDevices)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // ##################### QUEUE ######################
    // ##################################################

    println!("Command Queue:{b}\
            Context: {}{d}\
            Device: {}{d}\
            ReferenceCount: {}{d}\
            Properties: {}{e}\
        ",
        to_string!(core::get_command_queue_info(&queue, CommandQueueInfo::Context)),
        to_string!(core::get_command_queue_info(&queue, CommandQueueInfo::Device)),
        to_string!(core::get_command_queue_info(&queue, CommandQueueInfo::ReferenceCount)),
        to_string!(core::get_command_queue_info(&queue, CommandQueueInfo::Properties)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // ################### MEM OBJECT ###################
    // ##################################################

    println!("Buffer Memory:{b}\
            Type: {}{d}\
            Flags: {}{d}\
            Size: {}{d}\
            HostPtr: {}{d}\
            MapCount: {}{d}\
            ReferenceCount: {}{d}\
            Context: {}{d}\
            AssociatedMemobject: {}{d}\
            Offset: {}{e}\
        ",
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Type)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Flags)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Size)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::HostPtr)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::MapCount)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::ReferenceCount)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Context)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::AssociatedMemobject)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Offset)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // ##################### IMAGE ######################
    // ##################################################

    println!("Image: {b}\
            ElementSize: {}{d}\
            RowPitch: {}{d}\
            SlicePitch: {}{d}\
            Width: {}{d}\
            Height: {}{d}\
            Depth: {}{d}\
            ArraySize: {}{d}\
            Buffer: {}{d}\
            NumMipLevels: {}{d}\
            NumSamples: {}{e}\
        ",
        to_string!(core::get_image_info(&image, ImageInfo::ElementSize)),
        to_string!(core::get_image_info(&image, ImageInfo::RowPitch)),
        to_string!(core::get_image_info(&image, ImageInfo::SlicePitch)),
        to_string!(core::get_image_info(&image, ImageInfo::Width)),
        to_string!(core::get_image_info(&image, ImageInfo::Height)),
        to_string!(core::get_image_info(&image, ImageInfo::Depth)),
        to_string!(core::get_image_info(&image, ImageInfo::ArraySize)),
        to_string!(core::get_image_info(&image, ImageInfo::Buffer)),
        to_string!(core::get_image_info(&image, ImageInfo::NumMipLevels)),
        to_string!(core::get_image_info(&image, ImageInfo::NumSamples)),
        b = begin, d = delim, e = end,
    );

    println!("Image Memory:{b}\
            Type: {}{d}\
            Flags: {}{d}\
            Size: {}{d}\
            HostPtr: {}{d}\
            MapCount: {}{d}\
            ReferenceCount: {}{d}\
            Context: {}{d}\
            AssociatedMemobject: {}{d}\
            Offset: {}{e}\
        ",
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Type)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Flags)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Size)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::HostPtr)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::MapCount)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::ReferenceCount)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Context)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::AssociatedMemobject)),
        to_string!(core::get_mem_object_info(&buffer, MemInfo::Offset)),
        b = begin, d = delim, e = end,
    );

    // ##################################################
    // #################### SAMPLER #####################
    // ##################################################


    println!("Sampler:{b}\
            ReferenceCount: {}{d}\
            Context: {}{d}\
            NormalizedCoords: {}{d}\
            AddressingMode: {}{d}\
            FilterMode: {}{e}\
        ",
        to_string!(core::get_sampler_info(&sampler, SamplerInfo::ReferenceCount)),
        to_string!(core::get_sampler_info(&sampler, SamplerInfo::Context)),
        to_string!(core::get_sampler_info(&sampler, SamplerInfo::NormalizedCoords)),
        to_string!(core::get_sampler_info(&sampler, SamplerInfo::AddressingMode)),
        to_string!(core::get_sampler_info(&sampler, SamplerInfo::FilterMode)),
        b = begin, d = delim, e = end,
    );

    // ##################################################
    // #################### PROGRAM #####################
    // ##################################################

    println!("Program:{b}\
            ReferenceCount: {}{d}\
            Context: {}{d}\
            NumDevices: {}{d}\
            Devices: {}{d}\
            Source: {}{d}\
            BinarySizes: {}{d}\
            Binaries: {}{d}\
            NumKernels: {}{d}\
            KernelNames: {}{e}\
        ",
        to_string!(core::get_program_info(&program, ProgramInfo::ReferenceCount)),
        to_string!(core::get_program_info(&program, ProgramInfo::Context)),
        to_string!(core::get_program_info(&program, ProgramInfo::NumDevices)),
        to_string!(core::get_program_info(&program, ProgramInfo::Devices)),
        to_string!(core::get_program_info(&program, ProgramInfo::Source)),
        to_string!(core::get_program_info(&program, ProgramInfo::BinarySizes)
            .map(|_| "{Omitted}")),
        to_string!(core::get_program_info(&program, ProgramInfo::NumKernels)),
        to_string!(core::get_program_info(&program, ProgramInfo::KernelNames)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // ################# PROGRAM BUILD ##################
    // ##################################################

    println!("Program Build:{b}\
            BuildStatus: {}{d}\
            BuildOptions: {}{d}\
            BuildLog: \n\n{}{d}\n\
            BinaryType: {}{e}\
        ",
        to_string!(core::get_program_build_info(&program, &device, ProgramBuildInfo::BuildStatus)),
        to_string!(core::get_program_build_info(&program, &device, ProgramBuildInfo::BuildOptions)),
        to_string!(core::get_program_build_info(&program, &device, ProgramBuildInfo::BuildLog)),
        to_string!(core::get_program_build_info(&program, &device, ProgramBuildInfo::BinaryType)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // ##################### KERNEL #####################
    // ##################################################

    println!("Kernel Info:{b}\
            FunctionName: {}{d}\
            NumArgs: {}{d}\
            ReferenceCount: {}{d}\
            Context: {}{d}\
            Program: {}{d}\
            Attributes: {}{e}\
        ",
        to_string!(core::get_kernel_info(&kernel, KernelInfo::FunctionName)),
        to_string!(core::get_kernel_info(&kernel, KernelInfo::NumArgs)),
        to_string!(core::get_kernel_info(&kernel, KernelInfo::ReferenceCount)),
        to_string!(core::get_kernel_info(&kernel, KernelInfo::Context)),
        to_string!(core::get_kernel_info(&kernel, KernelInfo::Program)),
        to_string!(core::get_kernel_info(&kernel, KernelInfo::Attributes)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // ################# KERNEL ARGUMENT ################
    // ##################################################

    println!("Kernel Argument [0]:{b}\
            AddressQualifier: {}{d}\
            AccessQualifier: {}{d}\
            TypeName: {}{d}\
            TypeQualifier: {}{d}\
            Name: {}{e}\
        ",
        to_string!(core::get_kernel_arg_info(&kernel, 0, KernelArgInfo::AddressQualifier, Some(&[device_version]))),
        to_string!(core::get_kernel_arg_info(&kernel, 0, KernelArgInfo::AccessQualifier, Some(&[device_version]))),
        to_string!(core::get_kernel_arg_info(&kernel, 0, KernelArgInfo::TypeName, Some(&[device_version]))),
        to_string!(core::get_kernel_arg_info(&kernel, 0, KernelArgInfo::TypeQualifier, Some(&[device_version]))),
        to_string!(core::get_kernel_arg_info(&kernel, 0, KernelArgInfo::Name, Some(&[device_version]))),
        b = begin, d = delim, e = end,
    );

    // ##################################################
    // ################ KERNEL WORK GROUP ###############
    // ##################################################

    println!("Kernel Work Group:{b}\
            WorkGroupSize: {}{d}\
            CompileWorkGroupSize: {}{d}\
            LocalMemSize: {}{d}\
            PreferredWorkGroupSizeMultiple: {}{d}\
            PrivateMemSize: {}{d}\
            GlobalWorkSize: {}{e}\
        ",
        to_string!(core::get_kernel_work_group_info(&kernel, &device, KernelWorkGroupInfo::WorkGroupSize)),
        to_string!(core::get_kernel_work_group_info(&kernel, &device, KernelWorkGroupInfo::CompileWorkGroupSize)),
        to_string!(core::get_kernel_work_group_info(&kernel, &device, KernelWorkGroupInfo::LocalMemSize)),
        to_string!(core::get_kernel_work_group_info(&kernel, &device, KernelWorkGroupInfo::PreferredWorkGroupSizeMultiple)),
        to_string!(core::get_kernel_work_group_info(&kernel, &device, KernelWorkGroupInfo::PrivateMemSize)),
        to_string!(core::get_kernel_work_group_info(&kernel, &device, KernelWorkGroupInfo::GlobalWorkSize)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // ##################### EVENT ######################
    // ##################################################

    println!("Event:{b}\
            CommandQueue: {}{d}\
            CommandType: {}{d}\
            ReferenceCount: {}{d}\
            CommandExecutionStatus: {}{d}\
            Context: {}{e}\
        ",
        to_string!(core::get_event_info(&event, EventInfo::CommandQueue)),
        to_string!(core::get_event_info(&event, EventInfo::CommandType)),
        to_string!(core::get_event_info(&event, EventInfo::ReferenceCount)),
        to_string!(core::get_event_info(&event, EventInfo::CommandExecutionStatus)),
        to_string!(core::get_event_info(&event, EventInfo::Context)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // ################ EVENT PROFILING #################
    // ##################################################

    println!("Event Profiling:{b}\
            Queued: {}{d}\
            Submit: {}{d}\
            Start: {}{d}\
            End: {}{e}\
        ",
        to_string!(core::get_event_profiling_info(&event, ProfilingInfo::Queued)),
        to_string!(core::get_event_profiling_info(&event, ProfilingInfo::Submit)),
        to_string!(core::get_event_profiling_info(&event, ProfilingInfo::Start)),
        to_string!(core::get_event_profiling_info(&event, ProfilingInfo::End)),
        b = begin, d = delim, e = end,
    );


    // ##################################################
    // ###################### END #######################
    // ##################################################

    print!("\n");
    Ok(())
}

fn print_platform(plat_idx: usize, platform: Platform) -> ocl::Result<()> {
    for (device_idx, &device) in Device::list_all(&platform)?.iter().enumerate() {
        print_platform_device(plat_idx, platform, device_idx, device)?;
    }
    Ok(())
}

fn info_core() -> ocl::Result<()> {
    let platforms = Platform::list();
    for (plat_idx, &platform) in platforms.iter().enumerate() {
        print_platform(plat_idx, platform)?;
    }
    Ok(())
}

pub fn main() {
    match info_core() {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}

// ############### OpenCL Platform-Device Full Info ################

// Platform [0]:
// Profile: FULL_PROFILE
// Version: OpenCL 2.1
// Name: Intel(R) OpenCL
// Vendor: Intel(R) Corporation
// Extensions: cl_intel_dx9_media_sharing cl_khr_3d_image_writes cl_khr_byte_addressable_store cl_khr_d3d11_sharing cl_khr_depth_images cl_khr_dx9_media_sharing cl_khr_fp64 cl_khr_gl_sharing cl_khr_global_int32_base_atomics cl_khr_global_int32_extended_atomics cl_khr_icd cl_khr_image2d_from_buffer cl_khr_local_int32_base_atomics cl_khr_local_int32_extended_atomics cl_khr_spir

// Device [0]:
// Type: GPU
// VendorId: 32902
// MaxComputeUnits: 24
// MaxWorkItemDimensions: 3
// MaxWorkGroupSize: 256
// MaxWorkItemSizes: [256, 256, 256]
// PreferredVectorWidthChar: 16
// PreferredVectorWidthShort: 8
// PreferredVectorWidthInt: 4
// PreferredVectorWidthLong: 1
// PreferredVectorWidthFloat: 1
// PreferredVectorWidthDouble: 1
// MaxClockFrequency: 1050
// AddressBits: 64
// MaxReadImageArgs: 128
// MaxWriteImageArgs: 128
// MaxMemAllocSize: 3413303296
// Image2dMaxWidth: 16384
// Image2dMaxHeight: 16384
// Image3dMaxWidth: 16384
// Image3dMaxHeight: 16384
// Image3dMaxDepth: 2048
// ImageSupport: true
// MaxParameterSize: 1024
// MaxSamplers: 16
// MemBaseAddrAlign: 1024
// MinDataTypeAlignSize: 128
// SingleFpConfig: DENORM | INF_NAN | ROUND_TO_NEAREST | ROUND_TO_ZERO | ROUND_TO_INF | FMA | CORRECTLY_ROUNDED_DIVIDE_SQRTGlobalMemCacheType: ReadWriteCache
// GlobalMemCachelineSize: 64
// GlobalMemCacheSize: 524288
// GlobalMemSize: 6826606592
// MaxConstantBufferSize: 3413303296
// MaxConstantArgs: 8
// LocalMemType: Local
// LocalMemSize: 65536
// ErrorCorrectionSupport: false
// ProfilingTimerResolution: 83
// EndianLittle: true
// Available: true
// CompilerAvailable: true
// ExecutionCapabilities: KERNEL
// QueueProperties: OUT_OF_ORDER_EXEC_MODE_ENABLE | PROFILING_ENABLE
// Name: Intel(R) HD Graphics 520
// Vendor: Intel(R) Corporation
// DriverVersion: 23.20.16.4973
// Profile: FULL_PROFILE
// Version: 2.1
// Extensions: cl_khr_3d_image_writes cl_khr_byte_addressable_store cl_khr_fp16 cl_khr_depth_images cl_khr_global_int32_base_atomics cl_khr_global_int32_extended_atomics cl_khr_icd cl_khr_image2d_from_buffer cl_khr_local_int32_base_atomics cl_khr_local_int32_extended_atomics cl_intel_subgroups cl_intel_required_subgroup_size cl_intel_subgroups_short cl_khr_spir cl_intel_accelerator cl_intel_media_block_io cl_intel_driver_diagnostics cl_intel_device_side_avc_motion_estimation cl_khr_priority_hints cl_khr_subgroups cl_khr_il_program cl_khr_fp64 cl_intel_planar_yuv cl_intel_packed_yuv cl_intel_motion_estimation cl_intel_advanced_motion_estimation cl_khr_gl_sharing cl_khr_gl_depth_images cl_khr_gl_event cl_khr_gl_msaa_sharing cl_intel_dx9_media_sharing cl_khr_dx9_media_sharing cl_khr_d3d10_sharing cl_khr_d3d11_sharing cl_intel_d3d11_nv12_media_sharing cl_intel_simultaneous_sharing
// Platform: PlatformId(0x1cc3000)
// DoubleFpConfig: DENORM | INF_NAN | ROUND_TO_NEAREST | ROUND_TO_ZERO | ROUND_TO_INF | FMA
// HalfFpConfig: DENORM | INF_NAN | ROUND_TO_NEAREST | ROUND_TO_ZERO | ROUND_TO_INF | FMA
// PreferredVectorWidthHalf: 8
// HostUnifiedMemory: true
// NativeVectorWidthChar: 16
// NativeVectorWidthShort: 8
// NativeVectorWidthInt: 4
// NativeVectorWidthLong: 1
// NativeVectorWidthFloat: 1
// NativeVectorWidthDouble: 1
// NativeVectorWidthHalf: 8
// OpenclCVersion: OpenCL C 2.1
// LinkerAvailable: true
// BuiltInKernels: block_motion_estimate_intel;block_advanced_motion_estimate_check_intel;block_advanced_motion_estimate_bidirectional_check_intel;
// ImageMaxBufferSize: 213331456
// ImageMaxArraySize: 2048
// ParentDevice: None
// PartitionMaxSubDevices: 0
// PartitionProperties: []
// PartitionAffinityDomain: (empty)
// PartitionType: []
// ReferenceCount: 1
// PreferredInteropUserSync: true
// PrintfBufferSize: 4194304
// ImagePitchAlignment: 4
// ImageBaseAddressAlignment: 4

// Context:
// Reference Count: 1
// Devices: [DeviceId(0x1c37100)]
// Properties: ContextProperties { props: {Platform: Platform(PlatformId(0x1cc3000))}, contains_gl_context_or_sharegroup: false }
// Device Count: 1

// Command Queue:
// Context: Context(0x3ec3d60)
// Device: DeviceId(0x1c37100)
// ReferenceCount: 4
// Properties: PROFILING_ENABLE

// Buffer Memory:
// Type: Buffer
// Flags: READ_WRITE
// Size: 4194304
// HostPtr: None
// MapCount: 0
// ReferenceCount: 2
// Context: Context(0x3ec3d60)
// AssociatedMemobject: None
// Offset: 0

// Image:
// ElementSize: 4
// RowPitch: 4096
// SlicePitch: 0
// Width: 1024
// Height: 0
// Depth: 0
// ArraySize: 0
// Buffer: None
// NumMipLevels: 0
// NumSamples: 0

// Image Memory:
// Type: Buffer
// Flags: READ_WRITE
// Size: 4194304
// HostPtr: None
// MapCount: 0
// ReferenceCount: 2
// Context: Context(0x3ec3d60)
// AssociatedMemobject: None
// Offset: 0

// Sampler:
// ReferenceCount: 1
// Context: Context(0x3ec3d60)
// NormalizedCoords: false
// AddressingMode: None
// FilterMode: Nearest

// Program:
// ReferenceCount: 2
// Context: Context(0x3ec3d60)
// NumDevices: 1
// Devices: [DeviceId(0x1c37100)]
// Source: __kernel void multiply(float coeff, __global float* buffer) {
//         buffer[get_global_id(0)] *= coeff;
//     }
// BinarySizes: {Omitted}
// Binaries: 1
// NumKernels: multiply
// KernelNames:


// Program Build:
// BuildStatus: Success
// BuildOptions:
// BuildLog:



// BinaryType: EXECUTABLE

// Kernel Info:
// FunctionName: multiply
// NumArgs: 2
// ReferenceCount: 1
// Context: Context(0x3ec3d60)
// Program: Program(0x1caf860)
// Attributes:

// Kernel Argument [0]:
// AddressQualifier: Private
// AccessQualifier: None
// TypeName: float
// TypeQualifier: NONE
// Name: coeff

// Kernel Work Group:
// WorkGroupSize: 256
// CompileWorkGroupSize: [0, 0, 0]
// LocalMemSize: 0
// PreferredWorkGroupSizeMultiple: 32
// PrivateMemSize: 0
// GlobalWorkSize: only available for custom devices or built-in kernels

// Event:
// CommandQueue: CommandQueue(0x1c3c5d0)
// CommandType: WriteBuffer
// ReferenceCount: 1
// CommandExecutionStatus: Complete
// Context: Context(0x3ec3d60)

// Event Profiling:
// Queued: 1491212172800
// Submit: 1491212180200
// Start: 1491212205500
// End: 1491227660000


// ############### OpenCL Platform-Device Full Info ################

// Platform [0]:
// Profile: FULL_PROFILE
// Version: OpenCL 2.1
// Name: Intel(R) OpenCL
// Vendor: Intel(R) Corporation
// Extensions: cl_intel_dx9_media_sharing cl_khr_3d_image_writes cl_khr_byte_addressable_store cl_khr_d3d11_sharing cl_khr_depth_images cl_khr_dx9_media_sharing cl_khr_fp64 cl_khr_gl_sharing cl_khr_global_int32_base_atomics cl_khr_global_int32_extended_atomics cl_khr_icd cl_khr_image2d_from_buffer cl_khr_local_int32_base_atomics cl_khr_local_int32_extended_atomics cl_khr_spir

// Device [1]:
// Type: CPU
// VendorId: 32902
// MaxComputeUnits: 4
// MaxWorkItemDimensions: 3
// MaxWorkGroupSize: 8192
// MaxWorkItemSizes: [8192, 8192, 8192]
// PreferredVectorWidthChar: 1
// PreferredVectorWidthShort: 1
// PreferredVectorWidthInt: 1
// PreferredVectorWidthLong: 1
// PreferredVectorWidthFloat: 1
// PreferredVectorWidthDouble: 1
// MaxClockFrequency: 2500
// AddressBits: 64
// MaxReadImageArgs: 480
// MaxWriteImageArgs: 480
// MaxMemAllocSize: 4266629120
// Image2dMaxWidth: 16384
// Image2dMaxHeight: 16384
// Image3dMaxWidth: 2048
// Image3dMaxHeight: 2048
// Image3dMaxDepth: 2048
// ImageSupport: true
// MaxParameterSize: 3840
// MaxSamplers: 480
// MemBaseAddrAlign: 1024
// MinDataTypeAlignSize: 128
// SingleFpConfig: DENORM | INF_NAN | ROUND_TO_NEAREST
// GlobalMemCacheType: ReadWriteCache
// GlobalMemCachelineSize: 64
// GlobalMemCacheSize: 262144
// GlobalMemSize: 17066516480
// MaxConstantBufferSize: 131072
// MaxConstantArgs: 480
// LocalMemType: Global
// LocalMemSize: 32768
// ErrorCorrectionSupport: false
// ProfilingTimerResolution: 100
// EndianLittle: true
// Available: true
// CompilerAvailable: true
// ExecutionCapabilities: KERNEL | NATIVE_KERNEL
// QueueProperties: OUT_OF_ORDER_EXEC_MODE_ENABLE | PROFILING_ENABLE
// Name: Intel(R) Core(TM) i7-6500U CPU @ 2.50GHz
// Vendor: Intel(R) Corporation
// DriverVersion: 7.6.0.611
// Profile: FULL_PROFILE
// Version: 2.1
// Extensions: cl_khr_icd cl_khr_global_int32_base_atomics cl_khr_global_int32_extended_atomics cl_khr_local_int32_base_atomics cl_khr_local_int32_extended_atomics cl_khr_byte_addressable_store cl_khr_depth_images cl_khr_3d_image_writes cl_intel_exec_by_local_thread cl_khr_spir cl_khr_dx9_media_sharing cl_intel_dx9_media_sharing cl_khr_d3d11_sharing cl_khr_gl_sharing cl_khr_fp64 cl_khr_image2d_from_buffer cl_intel_vec_len_hint
// Platform: PlatformId(0x1cc3000)
// DoubleFpConfig: DENORM | INF_NAN | ROUND_TO_NEAREST | ROUND_TO_ZERO | ROUND_TO_INF | FMA
// HalfFpConfig: <unavailable (CL_INVALID_VALUE)>
// PreferredVectorWidthHalf: 0
// HostUnifiedMemory: true
// NativeVectorWidthChar: 32
// NativeVectorWidthShort: 16
// NativeVectorWidthInt: 8
// NativeVectorWidthLong: 4
// NativeVectorWidthFloat: 8
// NativeVectorWidthDouble: 4
// NativeVectorWidthHalf: 0
// OpenclCVersion: OpenCL C 2.0
// LinkerAvailable: true
// BuiltInKernels: Device info unavailable
// ImageMaxBufferSize: 266664320
// ImageMaxArraySize: 2048
// ParentDevice: None
// PartitionMaxSubDevices: 4
// PartitionProperties: []
// PartitionAffinityDomain: (empty)
// PartitionType: Device info unavailable
// ReferenceCount: 1
// PreferredInteropUserSync: false
// PrintfBufferSize: 1048576
// ImagePitchAlignment: 64
// ImageBaseAddressAlignment: 64

// Context:
// Reference Count: 1
// Devices: [DeviceId(0x1cdaa70)]
// Properties: ContextProperties { props: {Platform: Platform(PlatformId(0x1cc3000))}, contains_gl_context_or_sharegroup: false }
// Device Count: 1

// Command Queue:
// Context: Context(0x1caf860)
// Device: DeviceId(0x1cdaa70)
// ReferenceCount: 4
// Properties: PROFILING_ENABLE

// Buffer Memory:
// Type: Buffer
// Flags: READ_WRITE
// Size: 4194304
// HostPtr: None
// MapCount: 0
// ReferenceCount: 2
// Context: Context(0x1caf860)
// AssociatedMemobject: None
// Offset: 0

// Image:
// ElementSize: 4
// RowPitch: 4096
// SlicePitch: 0
// Width: 1024
// Height: 0
// Depth: 0
// ArraySize: 0
// Buffer: None
// NumMipLevels: 0
// NumSamples: 0

// Image Memory:
// Type: Buffer
// Flags: READ_WRITE
// Size: 4194304
// HostPtr: None
// MapCount: 0
// ReferenceCount: 2
// Context: Context(0x1caf860)
// AssociatedMemobject: None
// Offset: 0

// Sampler:
// ReferenceCount: 1
// Context: Context(0x1caf860)
// NormalizedCoords: false
// AddressingMode: None
// FilterMode: Nearest

// Program:
// ReferenceCount: 1
// Context: Context(0x1caf860)
// NumDevices: 1
// Devices: [DeviceId(0x1cdaa70)]
// Source: __kernel void multiply(float coeff, __global float* buffer) {
//         buffer[get_global_id(0)] *= coeff;
//     }
// BinarySizes: {Omitted}
// Binaries: 1
// NumKernels: multiply
// KernelNames:


// Program Build:
// BuildStatus: Success
// BuildOptions:
// BuildLog:

// Compilation started
// Compilation done
// Linking started
// Linking done
// Device build started
// Device build done
// Kernel <multiply> was successfully vectorized (8)
// Done.

// BinaryType: EXECUTABLE

// Kernel Info:
// FunctionName: multiply
// NumArgs: 2
// ReferenceCount: 1
// Context: Context(0x1caf860)
// Program: Program(0x3ef3640)
// Attributes:

// Kernel Argument [0]:
// AddressQualifier: Private
// AccessQualifier: None
// TypeName: float
// TypeQualifier: NONE
// Name: coeff

// Kernel Work Group:
// WorkGroupSize: Kernel work-group info unavailable
// CompileWorkGroupSize: Kernel work-group info unavailable
// LocalMemSize: Kernel work-group info unavailable
// PreferredWorkGroupSizeMultiple: Kernel work-group info unavailable
// PrivateMemSize: Kernel work-group info unavailable
// GlobalWorkSize: only available for custom devices or built-in kernels

// Event:
// CommandQueue: CommandQueue(0x54c9640)
// CommandType: WriteBuffer
// ReferenceCount: 1
// CommandExecutionStatus: Complete
// Context: Context(0x1caf860)

// Event Profiling:
// Queued: 115865446302800
// Submit: 115865446311900
// Start: 115865446387200
// End: 115865468413600


// ############### OpenCL Platform-Device Full Info ################

// Platform [1]:
// Profile: FULL_PROFILE
// Version: OpenCL 2.1 AMD-APP (2841.19)
// Name: AMD Accelerated Parallel Processing
// Vendor: Advanced Micro Devices, Inc.
// Extensions: cl_khr_icd cl_khr_d3d10_sharing cl_khr_d3d11_sharing cl_khr_dx9_media_sharing cl_amd_event_callback cl_amd_offline_devices

// Device [0]:
// Type: GPU
// VendorId: 4098
// MaxComputeUnits: 6
// MaxWorkItemDimensions: 3
// MaxWorkGroupSize: 256
// MaxWorkItemSizes: [1024, 1024, 1024]
// PreferredVectorWidthChar: 4
// PreferredVectorWidthShort: 2
// PreferredVectorWidthInt: 1
// PreferredVectorWidthLong: 1
// PreferredVectorWidthFloat: 1
// PreferredVectorWidthDouble: 1
// MaxClockFrequency: 400
// AddressBits: 64
// MaxReadImageArgs: 128
// MaxWriteImageArgs: 64
// MaxMemAllocSize: 1597190963
// Image2dMaxWidth: 16384
// Image2dMaxHeight: 16384
// Image3dMaxWidth: 2048
// Image3dMaxHeight: 2048
// Image3dMaxDepth: 2048
// ImageSupport: true
// MaxParameterSize: 1024
// MaxSamplers: 16
// MemBaseAddrAlign: 2048
// MinDataTypeAlignSize: 128
// SingleFpConfig: INF_NAN | ROUND_TO_NEAREST | ROUND_TO_ZERO | ROUND_TO_INF | FMA | CORRECTLY_ROUNDED_DIVIDE_SQRT
// GlobalMemCacheType: ReadWriteCache
// GlobalMemCachelineSize: 64
// GlobalMemCacheSize: 16384
// GlobalMemSize: 2147483648
// MaxConstantBufferSize: 1597190963
// MaxConstantArgs: 8
// LocalMemType: Local
// LocalMemSize: 32768
// ErrorCorrectionSupport: false
// ProfilingTimerResolution: 1
// EndianLittle: true
// Available: true
// CompilerAvailable: true
// ExecutionCapabilities: KERNEL
// QueueProperties: PROFILING_ENABLE
// Name: Iceland
// Vendor: Advanced Micro Devices, Inc.
// DriverVersion: 2841.19
// Profile: FULL_PROFILE
// Version: 2.0
// Extensions: cl_khr_fp64 cl_amd_fp64 cl_khr_global_int32_base_atomics cl_khr_global_int32_extended_atomics cl_khr_local_int32_base_atomics cl_khr_local_int32_extended_atomics cl_khr_int64_base_atomics cl_khr_int64_extended_atomics cl_khr_3d_image_writes cl_khr_byte_addressable_store cl_khr_fp16 cl_khr_gl_sharing cl_khr_gl_depth_images cl_amd_device_attribute_query cl_amd_vec3 cl_amd_printf cl_amd_media_ops cl_amd_media_ops2 cl_amd_popcnt cl_khr_d3d10_sharing cl_khr_d3d11_sharing cl_khr_dx9_media_sharing cl_khr_image2d_from_buffer cl_khr_spir cl_khr_subgroups cl_khr_gl_event cl_khr_depth_images cl_khr_mipmap_image cl_khr_mipmap_image_writes cl_amd_liquid_flash cl_amd_planar_yuv
// Platform: PlatformId(0x7ffa0be32fd0)
// DoubleFpConfig: DENORM | INF_NAN | ROUND_TO_NEAREST | ROUND_TO_ZERO | ROUND_TO_INF | FMA
// HalfFpConfig: (empty)
// PreferredVectorWidthHalf: 1
// HostUnifiedMemory: false
// NativeVectorWidthChar: 4
// NativeVectorWidthShort: 2
// NativeVectorWidthInt: 1
// NativeVectorWidthLong: 1
// NativeVectorWidthFloat: 1
// NativeVectorWidthDouble: 1
// NativeVectorWidthHalf: 1
// OpenclCVersion: OpenCL C 2.0
// LinkerAvailable: true
// BuiltInKernels:
// ImageMaxBufferSize: 134217728
// ImageMaxArraySize: 2048
// ParentDevice: None
// PartitionMaxSubDevices: 6
// PartitionProperties: []
// PartitionAffinityDomain: (empty)
// PartitionType: []
// ReferenceCount: 1
// PreferredInteropUserSync: true
// PrintfBufferSize: 4194304
// ImagePitchAlignment: 256
// ImageBaseAddressAlignment: 256

// Context:
// Reference Count: 5
// Devices: [DeviceId(0x3ecc980)]
// Properties: ContextProperties { props: {Platform: Platform(PlatformId(0x7ffa0be32fd0))}, contains_gl_context_or_sharegroup: false }
// Device Count: 1

// Command Queue:
// Context: Context(0x3f90b40)
// Device: DeviceId(0x3ecc980)
// ReferenceCount: 4
// Properties: PROFILING_ENABLE

// Buffer Memory:
// Type: Buffer
// Flags: READ_WRITE
// Size: 4194304
// HostPtr: None
// MapCount: 0
// ReferenceCount: 2
// Context: Context(0x3f90b40)
// AssociatedMemobject: None
// Offset: 0

// Image:
// ElementSize: 4
// RowPitch: 4096
// SlicePitch: 0
// Width: 1024
// Height: 0
// Depth: 0
// ArraySize: 0
// Buffer: None
// NumMipLevels: 0
// NumSamples: 0

// Image Memory:
// Type: Buffer
// Flags: READ_WRITE
// Size: 4194304
// HostPtr: None
// MapCount: 0
// ReferenceCount: 2
// Context: Context(0x3f90b40)
// AssociatedMemobject: None
// Offset: 0

// Sampler:
// ReferenceCount: 1
// Context: Context(0x3f90b40)
// NormalizedCoords: false
// AddressingMode: None
// FilterMode: Nearest

// Program:
// ReferenceCount: 2
// Context: Context(0x3f90b40)
// NumDevices: 1
// Devices: [DeviceId(0x3ecc980)]
// Source: __kernel void multiply(float coeff, __global float* buffer) {
//         buffer[get_global_id(0)] *= coeff;
//     }
// BinarySizes: {Omitted}
// Binaries: 1
// NumKernels: multiply
// KernelNames:


// Program Build:
// BuildStatus: Success
// BuildOptions:
// BuildLog:



// BinaryType: EXECUTABLE

// Kernel Info:
// FunctionName: multiply
// NumArgs: 2
// ReferenceCount: 1
// Context: Context(0x3f90b40)
// Program: Program(0x3ec1580)
// Attributes:

// Kernel Argument [0]:
// AddressQualifier: Private
// AccessQualifier: None
// TypeName: float
// TypeQualifier: NONE
// Name: coeff

// Kernel Work Group:
// WorkGroupSize: 256
// CompileWorkGroupSize: [0, 0, 0]
// LocalMemSize: 0
// PreferredWorkGroupSizeMultiple: 64
// PrivateMemSize: 0
// GlobalWorkSize: only available for custom devices or built-in kernels

// Event:
// CommandQueue: CommandQueue(0x3f44230)
// CommandType: WriteBuffer
// ReferenceCount: 1
// CommandExecutionStatus: Complete
// Context: Context(0x3f90b40)

// Event Profiling:
// Queued: 115875377774600
// Submit: 115875377824900
// Start: 115875394328826
// End: 115875401096900
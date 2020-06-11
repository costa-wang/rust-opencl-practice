#[macro_use] extern crate colorify;
extern crate ocl;
use ocl::{Result as OclResult, Platform, Device, Context, Image};
use ocl::enums::MemObjectType;

fn img_formats() -> OclResult<()> {
    for (p_idx, platform) in Platform::list().into_iter().enumerate() {
        for (d_idx, device) in Device::list_all(&platform)?.into_iter().enumerate() {
            printlnc!(blue: "Platform [{}]: {}", p_idx, platform.name()?);
            printlnc!(teal: "Device [{}]: {} {}", d_idx, device.vendor()?, device.name()?);

            let context = Context::builder().platform(platform).devices(device).build()?;

            let sup_img_formats = Image::<u8>::supported_formats(&context, ocl::flags::MEM_READ_WRITE,
                MemObjectType::Image2d)?;

            println!("Image Formats: {:#?}.", sup_img_formats);
        }
    }
    Ok(())
}

pub fn main() {
    match img_formats() {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}


// Platform [0]: Intel(R) OpenCL
// Device [0]: Intel(R) Corporation Intel(R) HD Graphics 520
// Image Formats: [
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Bgra,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Depth,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Depth,
//             channel_data_type: UnormInt16,
//         },
//     ),
// ].
// Platform [0]: Intel(R) OpenCL
// Device [1]: Intel(R) Corporation Intel(R) Core(TM) i7-6500U CPU @ 2.50GHz
// Image Formats: [
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Bgra,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Depth,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Depth,
//             channel_data_type: UnormInt16,
//         },
//     ),
// ].
// Platform [1]: AMD Accelerated Parallel Processing
// Device [0]: Advanced Micro Devices, Inc. Iceland
// Image Formats: [
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: R,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: SnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: A,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rg,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: SignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnsignedInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: UnsignedInt32,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgba,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Argb,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Argb,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Argb,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Argb,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Bgra,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Bgra,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Bgra,
//             channel_data_type: SignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Bgra,
//             channel_data_type: UnsignedInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: SnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Luminance,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: SnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: SnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: UnormInt8,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: HalfFloat,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Intensity,
//             channel_data_type: Float,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Rgb,
//             channel_data_type: UnormInt101010,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Depth,
//             channel_data_type: UnormInt16,
//         },
//     ),
//     Ok(
//         ImageFormat {
//             channel_order: Depth,
//             channel_data_type: Float,
//         },
//     ),
// ]
use onednn_sys::*;
#[cfg(feature="ocl")]
use ocl::{Platform, Device, Context, enums::{DeviceInfo, DeviceInfoResult}, flags::DeviceType};

#[test]
fn test_engine_create_cpu() {
    let mut engine = std::ptr::null_mut();
    let status = unsafe { 
        dnnl_engine_create(&mut engine as *mut *mut dnnl_engine, dnnl_engine_kind_t::dnnl_cpu, 0)
    };
    assert_eq!(status, dnnl_status_t::dnnl_success);
    let status = unsafe { dnnl_engine_destroy(engine) };
    assert_eq!(status, dnnl_status_t::dnnl_success);
}

#[cfg(feature="ocl")]
fn get_gpu_context() -> Option<Context> {
    for platform in Platform::list() {
        if let Ok(devices) = Device::list(&platform, Some(DeviceType::new().gpu())) {
            for device in devices {
                if let Ok(context) = Context::builder()
                    .platform(platform)
                    .devices(device)
                    .build() {
                    return Some(context);
                } 
            }
        }
    }
    None
} 

#[cfg(feature="ocl")]
#[test]
fn test_engine_create_ocl() {
    if let Some(context) = get_gpu_context() {
        let device = context.devices()[0];
        let device_type = match device.info(DeviceInfo::Type) {
            Ok(DeviceInfoResult::Type(device_type)) => device_type,
            _ => panic!()
        };
        let engine_kind = if device_type == DeviceType::new().gpu() {
            dnnl_engine_kind_t::dnnl_gpu
        }
        else {
            unimplemented!()
        };
        let mut engine = std::ptr::null_mut();
        let status = unsafe { 
            dnnl_engine_create_ocl(
                &mut engine as *mut *mut dnnl_engine, 
                engine_kind,
                device.as_raw() as onednn_sys::cl_device_id, 
                context.as_ptr() as onednn_sys::cl_context
            )
        };
        assert_eq!(status, dnnl_status_t::dnnl_success);
        let status = unsafe { dnnl_engine_destroy(engine) };
        assert_eq!(status, dnnl_status_t::dnnl_success);
    }
}

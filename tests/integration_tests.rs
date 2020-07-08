use onednn_sys::*;
#[cfg(feature="ocl")]
use ocl::Context;

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
#[test]
fn test_engine_create_ocl() {
    let context = Context::builder()
        .build()
        .unwrap();
    let device = context.devices()[0];
    let mut engine = std::ptr::null_mut();
    let status = unsafe { 
        dnnl_engine_create_ocl(
            &mut engine as *mut *mut dnnl_engine, 
            dnnl_engine_kind_t::dnnl_gpu, 
            device.as_raw() as onednn_sys::cl_device_id, 
            context.as_ptr() as onednn_sys::cl_context
        )
    };
    assert_eq!(status, dnnl_status_t::dnnl_success);
    let status = unsafe { dnnl_engine_destroy(engine) };
    assert_eq!(status, dnnl_status_t::dnnl_success);
}

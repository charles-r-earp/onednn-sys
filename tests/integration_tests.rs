use onednn_sys::*;

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

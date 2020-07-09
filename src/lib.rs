#![allow(warnings)]

#[cfg(not(feature = "opencl"))]
include!{"bindings.rs"}
#[cfg(feature = "opencl")]
include!{"bindings_opencl.rs"}


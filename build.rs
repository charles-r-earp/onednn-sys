use std::{error::Error, env, fs::{self, File}, io::{BufRead, BufReader}, path::{Path, PathBuf}};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        if cfg!(feature = "opencl") {
            out_dir.join("opencl")
        }
        else {
            out_dir
        }
    };
    #[allow(unused_must_use)]
    fs::create_dir(&out_dir);
    let gpu_runtime = if cfg!(feature = "opencl") {
        "OCL"   
    }
    else {
        "NONE"
    };
    let dst = cmake::Config::new("oneDNN")
        .define("DNNL_LIBRARY_TYPE", "STATIC")
        .define("DNNL_BUILD_EXAMPLES", "OFF")
        .define("DNNL_BUILD_TESTS", "OFF")
        .define("DNNL_CPU_RUNTIME", "OMP")
        .define("DNNL_GPU_RUNTIME", gpu_runtime)
        .out_dir(out_dir)
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib64").display()
    );
    println!("cargo:rustc-link-lib=static=dnnl");
    let mut omp_library = None;
    let mut omp_lib_name = None;
    let mut ocl_library = None;
    
    let cmake_cache_path = dst.join("build").join("CMakeCache.txt");
    let cmake_cache_reader = BufReader::new(File::open(cmake_cache_path)?);
  
    let omp_lib_name_pfx = "OpenMP_CXX_LIB_NAMES:STRING=";
    let ocl_library_pfx = "OpenCL_LIBRARY:FILEPATH=";
    
    let mut omp_library_pfx = None;
    
    let mut cmake_cache = String::new();
    
    for line in cmake_cache_reader.lines()
        .filter_map(|line| line.ok()) { 
        cmake_cache.push_str(&format!("{}\n", line));
            
        if line.starts_with(ocl_library_pfx) {
            ocl_library.replace(String::from(&line[ocl_library_pfx.len()..]));
        }
        else if line.starts_with(omp_lib_name_pfx) {
            let _omp_lib_name = line[omp_lib_name_pfx.len()..]
                .split(";")
                .take(1)
                .next();
            if let Some(_omp_lib_name) = _omp_lib_name {
                omp_lib_name.replace(String::from(_omp_lib_name));
                omp_library_pfx.replace(format!("OpenMP_{}_LIBRARY:FILEPATH=", _omp_lib_name));
            }
        }
        else if let Some(ref omp_library_pfx) = omp_library_pfx {
            if line.starts_with(&*omp_library_pfx) {
                omp_library.replace(String::from(&line[omp_library_pfx.len()..]));
            }
        }
    }
    println!("cargo:warning={}", cmake_cache);
    
    if let (Some(omp_library), Some(omp_lib_name)) = (omp_library, omp_lib_name) {
        let omp_link_path = Path::new(&omp_library)
            .parent()
            .unwrap();
        println!("cargo:rustc-link-search={}", omp_link_path.to_str().unwrap());
        println!("cargo:rustc-link-lib={}", omp_lib_name);
    }
    else {
        println!("cargo:warning=Unable to use OpenMP, running in SEQ mode. Performance on cpu will be signficantly reduced."); 
    }
    if cfg!(feature="opencl") {
        let ocl_library = ocl_library.expect("OpenCL not found!");
        let ocl_link_path = Path::new(&ocl_library)
            .parent()
            .unwrap();
        println!("cargo:rustc-link-search={}", ocl_link_path.to_str().unwrap());
        println!("cargo:rustc-link-lib=OpenCL");
    }
    
    #[cfg(feature = "bindgen")]
    {
        let bindings = bindgen::Builder::default()
            .header("wrapper.hpp")
            .clang_arg("--std")
            .clang_arg("c++14")
            .clang_arg("-I")
            .clang_arg(dst.join("include").display().to_string())
            .ctypes_prefix("::libc")
            .generate_block(false)
            .size_t_is_usize(true)
            .rustified_non_exhaustive_enum("dnnl.*")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .rustfmt_bindings(true)
            .generate()
            .expect("Unable to create bindings.");
        let bindings_file = if cfg!(feature = "opencl") {
            "src/bindings_opencl.rs"
        }
        else {
            "src/bindings.rs"
        };
        bindings.write_to_file(bindings_file).unwrap();
    }
    
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=c++");
    }
    else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    }
    else {
        println!("cargo:rustc-link-lib=c++");
        //println!("cargo:rustc-link-lib=stdc++");
    };
    
    
    
    println!("cargo:include={}", dst.join("include").display().to_string());
    
    println!("cargo:rustc-rerun-if-changed=wrapper.hpp");
    
    Ok(())
}

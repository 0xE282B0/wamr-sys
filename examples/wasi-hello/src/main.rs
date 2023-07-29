use wamr_sys::*;

const DEFAULT_HEAP_SIZE: u32 = 20971520;
const DEFAULT_STACK_SIZE: u32 = 163840;
const DEFAULT_ERROR_BUF_SIZE: usize = 128;

fn main() {

    let payload = include_bytes!("../wasi-hello-world.wasm");
    let payload_ptr = payload.clone().as_mut_ptr();

    let mut error_buf = [0i8; DEFAULT_ERROR_BUF_SIZE];

    let ret = unsafe { wasm_runtime_init() };
    assert!(ret);

    let module = unsafe {
        wasm_runtime_load(
            payload_ptr,
            payload.len() as u32,
            error_buf.as_mut_ptr(),
            error_buf.len() as u32,
        )
    };

    assert!((module as usize) != 0);

    unsafe {
        wamr_sys::wasm_runtime_set_wasi_args(
            module,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
            0,
        );
    }



    let module_instance = unsafe {
        wasm_runtime_instantiate(
            module,
            DEFAULT_STACK_SIZE,
            DEFAULT_HEAP_SIZE,
            error_buf.as_mut_ptr(),
            error_buf.len() as u32,
        )
    };

    //let err_u8_vec: Vec<u8> = error_buf.iter().map(|&x| x as u8).filter(|x| *x > 31 && *x < 124).collect();
    //print!("error {:?}", String::from_utf8(err_u8_vec));

    assert!((module_instance as usize) != 0);

    let _exec_env = unsafe { wasm_runtime_create_exec_env(module_instance, DEFAULT_STACK_SIZE) };

    let success = unsafe { wasm_application_execute_main(module_instance, 0, std::ptr::null_mut()) };

    assert!(success);

    let _main_result = unsafe { wasm_runtime_get_wasi_exit_code(module_instance) };
}

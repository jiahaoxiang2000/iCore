//! batch subsystem

use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use lazy_static::*;
use crate::loader::*;


struct AppManager {
    num_app: usize,
    current_app: usize,
}

impl AppManager {
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} ",
                i
            );
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {

        if self.current_app >= self.num_app {
            println!("All applications completed!");
            shutdown(false);
        }
        self.current_app += 1;
    }
}

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
             
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            load_apps(); // load apps
          
            AppManager {
                num_app,
                current_app: 0
            }
        })
    };
}

/// init batch subsystem
pub fn init() {
    print_app_info();

}

/// print apps info
pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

/// run next app
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
   
    app_manager.move_to_next_app();
    drop(app_manager);
    // before this we have to drop local variables related to resources manually
    // and release the resources
    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        __restore(init_app_cx(current_app));
    }
    panic!("Unreachable in batch::run_current_app!");
}

#[cfg(target_os="macos")]
use core_foundation::runloop::{CFRunLoop, kCFRunLoopCommonModes};
#[cfg(target_os="macos")]
use core_graphics::{
    event::{CGEvent, CGEventTapLocation, CGEventType},
    event::{CGEventTap, CGEventTapOptions, CGEventTapPlacement},
};


#[cfg(target_os="macos")]
pub fn mac_keyboard_event<T>(eventType: Vec<CGEventType>, callback: T) 
    where T: Fn(&CGEvent)
{      
    let current = CFRunLoop::get_current();

    let event_tap = CGEventTap::new(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        eventType,
        |_a, _b, d| {
            callback(&d);
            None
        },
    );
    match event_tap {     
        Ok(tap) => unsafe {
            let loop_source = tap
                .mach_port
                .create_runloop_source(0)
                .expect("Somethings is bad ");
            current.add_source(&loop_source, kCFRunLoopCommonModes);
            tap.enable();
            CFRunLoop::run_current();
        },
        Err(_) => assert!(false),
    };
}

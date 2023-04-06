#[cfg(target_os="macos")]
use core_foundation::runloop::{CFRunLoop, kCFRunLoopCommonModes};
#[cfg(target_os="macos")]
use core_graphics::{
    event::{CGEvent, CGEventTapLocation, CGEventType, EventField},
    event::{CGEventTap, CGEventTapOptions, CGEventTapPlacement, CGEventTapProxy},
};


#[cfg(target_os="macos")]
pub fn mac_keyboard_event<T>(callback: T) where T: Fn(&CGEvent) {  
    let event_tap_options = CGEventTapOptions::Default;
    let event_tap_location = CGEventTapLocation::HID;
    let event_tap_placement = CGEventTapPlacement::HeadInsertEventTap;
    let current = CFRunLoop::get_current();
    let event_tap = CGEventTap::new(
        event_tap_location,
        event_tap_placement,
        event_tap_options,
        vec![CGEventType::KeyDown, CGEventType::KeyUp],
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
        Err(_) => (assert!(false)),
    };
}

#[macro_export]
macro_rules! ip {
    ($($arg:tt)*) => {{
        cortex_m::interrupt::free(|cs| {
            if let Some(itm) = ITM_GLOBAL.borrow(cs).borrow_mut().as_mut() {
                //cortex_m::iprintln!(&mut itm.stim[0], $($arg)*);
            }
        });
    }};
}

#[doc(hidden)]
#[derive(Default)]
pub(crate) struct Callbacks<'a> {
    pub(crate) firdespm_callback: Option<Box<dyn FnMut(f64, &mut f64, &mut f64) -> i8 + 'a>>,
}

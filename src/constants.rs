#[allow(dead_code)]
#[cfg(windows)]
pub(crate) const LINE_ENDING: &'static str = "\r\n";

#[allow(dead_code)]
#[cfg(not(windows))]
pub(crate) const LINE_ENDING: &'static str = "\n";

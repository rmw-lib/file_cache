#[cfg(not(target_os = "windows"))]
pub fn get() -> Result<u64, std::io::Error> {
  rlimit::increase_nofile_limit(u64::MAX)
}

#[cfg(target_os = "windows")]
pub fn get() -> Result<u64, std::io::Error> {
  rlimit::setmaxstdio(2048)?;
  rlimit::getmaxstdio()
}

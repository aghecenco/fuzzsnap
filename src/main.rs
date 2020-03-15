#[macro_use]
extern crate afl;
extern crate snapshot;

use snapshot::version_map::VersionMap;
use snapshot::Snapshot;

fn main() {
    let mut vm = VersionMap::new();
    vm.new_version()
        .set_type_version("test".to_string(), 2)
        .new_version()
        .set_type_version("test".to_string(), 3)
        .new_version()
        .set_type_version("test".to_string(), 4);

    fuzz!(|data: &[u8]| {
        let mz = data.as_ptr() as *mut u8;
        let mut mzz: &[u8] = unsafe { std::slice::from_raw_parts_mut(mz, data.len()) };
        let _ = Snapshot::load(&mut mzz, vm.clone()).unwrap();
    });
}
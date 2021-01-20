pub struct DirCountUp;
pub struct DirCountDown;

pub trait DirToken {}
impl DirToken for DirCountUp {}
impl DirToken for DirCountDown {}

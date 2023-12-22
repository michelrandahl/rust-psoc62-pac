#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    struct_: [STRUCT; 16],
    _reserved1: [u8; 0x0e00],
    intr_struct: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x200 - IPC structure"]
    #[inline(always)]
    pub const fn struct_(&self, n: usize) -> &STRUCT {
        &self.struct_[n]
    }
    #[doc = "0x1000..0x1100 - IPC interrupt structure"]
    #[inline(always)]
    pub const fn intr_struct(&self, n: usize) -> &INTR_STRUCT {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4096)
                .add(32 * n)
                .cast()
        }
    }
}
#[doc = "IPC structure"]
pub use self::struct_::STRUCT;
#[doc = r"Cluster"]
#[doc = "IPC structure"]
pub mod struct_;
#[doc = "IPC interrupt structure"]
pub use self::intr_struct::INTR_STRUCT;
#[doc = r"Cluster"]
#[doc = "IPC interrupt structure"]
pub mod intr_struct;

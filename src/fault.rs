#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    struct_: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x1a0 - Fault structure"]
    #[inline(always)]
    pub const fn struct_(&self, n: usize) -> &STRUCT {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(256 * n)
                .cast()
        }
    }
}
#[doc = "Fault structure"]
pub use self::struct_::STRUCT;
#[doc = r"Cluster"]
#[doc = "Fault structure"]
pub mod struct_;

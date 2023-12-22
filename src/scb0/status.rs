#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `EC_BUSY` reader - Inidicates whether the externally clocked logic is potentially accessing the EZ memory (this is only possible in EZ and CMD_RESP mode). This bit can be used by SW to determine whether it is safe for the CPU to access the EZ memory (without bus wait states (a blocked CPU access) or bus errors being generated). Note that the INTR_TX.BLOCKED and INTR_RX.BLOCKED interrupt causes are used to indicate whether CPU access was actually blocked by externally clocked logic."]
pub type EC_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Inidicates whether the externally clocked logic is potentially accessing the EZ memory (this is only possible in EZ and CMD_RESP mode). This bit can be used by SW to determine whether it is safe for the CPU to access the EZ memory (without bus wait states (a blocked CPU access) or bus errors being generated). Note that the INTR_TX.BLOCKED and INTR_RX.BLOCKED interrupt causes are used to indicate whether CPU access was actually blocked by externally clocked logic."]
    #[inline(always)]
    pub fn ec_busy(&self) -> EC_BUSY_R {
        EC_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Generic status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

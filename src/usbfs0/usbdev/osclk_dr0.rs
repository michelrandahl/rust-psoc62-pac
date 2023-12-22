#[doc = "Register `OSCLK_DR0` reader"]
pub type R = crate::R<OSCLK_DR0_SPEC>;
#[doc = "Field `ADDER` reader - These bits return the lower 8 bits of the oscillator locking circuits adder output."]
pub type ADDER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits return the lower 8 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder(&self) -> ADDER_R {
        ADDER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Oscillator lock data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osclk_dr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCLK_DR0_SPEC;
impl crate::RegisterSpec for OSCLK_DR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osclk_dr0::R`](R) reader structure"]
impl crate::Readable for OSCLK_DR0_SPEC {}
#[doc = "`reset()` method sets OSCLK_DR0 to value 0"]
impl crate::Resettable for OSCLK_DR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

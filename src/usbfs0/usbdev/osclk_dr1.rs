#[doc = "Register `OSCLK_DR1` reader"]
pub type R = crate::R<OSCLK_DR1_SPEC>;
#[doc = "Field `ADDER_MSB` reader - These bits return the upper 7 bits of the oscillator locking circuits adder output."]
pub type ADDER_MSB_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - These bits return the upper 7 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder_msb(&self) -> ADDER_MSB_R {
        ADDER_MSB_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Oscillator lock data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osclk_dr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCLK_DR1_SPEC;
impl crate::RegisterSpec for OSCLK_DR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osclk_dr1::R`](R) reader structure"]
impl crate::Readable for OSCLK_DR1_SPEC {}
#[doc = "`reset()` method sets OSCLK_DR1 to value 0"]
impl crate::Resettable for OSCLK_DR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

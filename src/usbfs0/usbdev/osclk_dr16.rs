#[doc = "Register `OSCLK_DR16` reader"]
pub type R = crate::R<OSCLK_DR16_SPEC>;
#[doc = "Field `ADDER16` reader - These bits return the oscillator locking circuits adder output."]
pub type ADDER16_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - These bits return the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder16(&self) -> ADDER16_R {
        ADDER16_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "Oscillator lock data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osclk_dr16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSCLK_DR16_SPEC;
impl crate::RegisterSpec for OSCLK_DR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osclk_dr16::R`](R) reader structure"]
impl crate::Readable for OSCLK_DR16_SPEC {}
#[doc = "`reset()` method sets OSCLK_DR16 to value 0"]
impl crate::Resettable for OSCLK_DR16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `INTR_CAUSE3` reader"]
pub type R = crate::R<INTR_CAUSE3_SPEC>;
#[doc = "Field `PORT_INT` reader - Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
pub type PORT_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PORT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub fn port_int(&self) -> PORT_INT_R {
        PORT_INT_R::new(self.bits)
    }
}
#[doc = "Interrupt port cause register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_cause3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_CAUSE3_SPEC;
impl crate::RegisterSpec for INTR_CAUSE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_cause3::R`](R) reader structure"]
impl crate::Readable for INTR_CAUSE3_SPEC {}
#[doc = "`reset()` method sets INTR_CAUSE3 to value 0"]
impl crate::Resettable for INTR_CAUSE3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CLK_ECO_STATUS` reader"]
pub type R = crate::R<CLK_ECO_STATUS_SPEC>;
#[doc = "Field `ECO_OK` reader - Indicates the ECO internal oscillator circuit has sufficient amplitude. It may not meet the PPM accuracy or duty cycle spec."]
pub type ECO_OK_R = crate::BitReader;
#[doc = "Field `ECO_READY` reader - Indicates the ECO internal oscillator circuit has had enough time to fully stabilize. This is the output of a counter since ECO was enabled, and it does not check the ECO output. It is recommended to also confirm ECO_OK==1."]
pub type ECO_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates the ECO internal oscillator circuit has sufficient amplitude. It may not meet the PPM accuracy or duty cycle spec."]
    #[inline(always)]
    pub fn eco_ok(&self) -> ECO_OK_R {
        ECO_OK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the ECO internal oscillator circuit has had enough time to fully stabilize. This is the output of a counter since ECO was enabled, and it does not check the ECO output. It is recommended to also confirm ECO_OK==1."]
    #[inline(always)]
    pub fn eco_ready(&self) -> ECO_READY_R {
        ECO_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "ECO Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_eco_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_ECO_STATUS_SPEC;
impl crate::RegisterSpec for CLK_ECO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_eco_status::R`](R) reader structure"]
impl crate::Readable for CLK_ECO_STATUS_SPEC {}
#[doc = "`reset()` method sets CLK_ECO_STATUS to value 0"]
impl crate::Resettable for CLK_ECO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CM0_STATUS` reader"]
pub type R = crate::R<CM0_STATUS_SPEC>;
#[doc = "Field `SLEEPING` reader - Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
pub type SLEEPING_R = crate::BitReader;
#[doc = "Field `SLEEPDEEP` reader - Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
pub type SLEEPDEEP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
    #[inline(always)]
    pub fn sleeping(&self) -> SLEEPING_R {
        SLEEPING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "CM0+ status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM0_STATUS_SPEC;
impl crate::RegisterSpec for CM0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_status::R`](R) reader structure"]
impl crate::Readable for CM0_STATUS_SPEC {}
#[doc = "`reset()` method sets CM0_STATUS to value 0"]
impl crate::Resettable for CM0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `PWR_HIB_DATA[%s]` reader"]
pub type R = crate::R<PWR_HIB_DATA_SPEC>;
#[doc = "Register `PWR_HIB_DATA[%s]` writer"]
pub type W = crate::W<PWR_HIB_DATA_SPEC>;
#[doc = "Field `HIB_DATA` reader - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
pub type HIB_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `HIB_DATA` writer - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
pub type HIB_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn hib_data(&self) -> HIB_DATA_R {
        HIB_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    #[must_use]
    pub fn hib_data(&mut self) -> HIB_DATA_W<PWR_HIB_DATA_SPEC> {
        HIB_DATA_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HIBERNATE Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_hib_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_hib_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_HIB_DATA_SPEC;
impl crate::RegisterSpec for PWR_HIB_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_hib_data::R`](R) reader structure"]
impl crate::Readable for PWR_HIB_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_hib_data::W`](W) writer structure"]
impl crate::Writable for PWR_HIB_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_HIB_DATA[%s]
to value 0"]
impl crate::Resettable for PWR_HIB_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `FLASH_PWR_CTL` reader"]
pub type R = crate::R<FLASH_PWR_CTL_SPEC>;
#[doc = "Register `FLASH_PWR_CTL` writer"]
pub type W = crate::W<FLASH_PWR_CTL_SPEC>;
#[doc = "Field `ENABLE` reader - Controls 'enable' pin of the Flash memory."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Controls 'enable' pin of the Flash memory."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_HV` reader - Controls 'enable_hv' pin of the Flash memory."]
pub type ENABLE_HV_R = crate::BitReader;
#[doc = "Field `ENABLE_HV` writer - Controls 'enable_hv' pin of the Flash memory."]
pub type ENABLE_HV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable_hv(&self) -> ENABLE_HV_R {
        ENABLE_HV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<FLASH_PWR_CTL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    #[must_use]
    pub fn enable_hv(&mut self) -> ENABLE_HV_W<FLASH_PWR_CTL_SPEC> {
        ENABLE_HV_W::new(self, 1)
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
#[doc = "Flash power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_pwr_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_pwr_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_PWR_CTL_SPEC;
impl crate::RegisterSpec for FLASH_PWR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_pwr_ctl::R`](R) reader structure"]
impl crate::Readable for FLASH_PWR_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_pwr_ctl::W`](W) writer structure"]
impl crate::Writable for FLASH_PWR_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_PWR_CTL to value 0x03"]
impl crate::Resettable for FLASH_PWR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}

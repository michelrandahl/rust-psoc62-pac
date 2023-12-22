#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `LPREF_EN` reader - Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
pub type LPREF_EN_R = crate::BitReader;
#[doc = "Field `LPREF_EN` writer - Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
pub type LPREF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - - 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - - 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
    #[inline(always)]
    pub fn lpref_en(&self) -> LPREF_EN_R {
        LPREF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - - 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
    #[inline(always)]
    #[must_use]
    pub fn lpref_en(&mut self) -> LPREF_EN_W<CONFIG_SPEC> {
        LPREF_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - - 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<CONFIG_SPEC> {
        ENABLED_W::new(self, 31)
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
#[doc = "LPCOMP Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

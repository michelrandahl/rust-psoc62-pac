#[doc = "Register `CM0_CA_CTL0` reader"]
pub type R = crate::R<CM0_CA_CTL0_SPEC>;
#[doc = "Register `CM0_CA_CTL0` writer"]
pub type W = crate::W<CM0_CA_CTL0_SPEC>;
#[doc = "Field `RAM_ECC_EN` reader - Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
pub type RAM_ECC_EN_R = crate::BitReader;
#[doc = "Field `RAM_ECC_EN` writer - Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
pub type RAM_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_ECC_INJ_EN` reader - Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type RAM_ECC_INJ_EN_R = crate::BitReader;
#[doc = "Field `RAM_ECC_INJ_EN` writer - Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type RAM_ECC_INJ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAY` reader - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type WAY_R = crate::FieldReader;
#[doc = "Field `WAY` writer - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type WAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SET_ADDR` reader - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type SET_ADDR_R = crate::FieldReader;
#[doc = "Field `SET_ADDR` writer - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type SET_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PREF_EN` reader - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PREF_EN_R = crate::BitReader;
#[doc = "Field `PREF_EN` writer - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PREF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CA_EN` reader - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
pub type CA_EN_R = crate::BitReader;
#[doc = "Field `CA_EN` writer - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
pub type CA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    pub fn ram_ecc_en(&self) -> RAM_ECC_EN_R {
        RAM_ECC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn ram_ecc_inj_en(&self) -> RAM_ECC_INJ_EN_R {
        RAM_ECC_INJ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn way(&self) -> WAY_R {
        WAY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_addr(&self) -> SET_ADDR_R {
        SET_ADDR_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
    #[inline(always)]
    pub fn ca_en(&self) -> CA_EN_R {
        CA_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ram_ecc_en(&mut self) -> RAM_ECC_EN_W<CM0_CA_CTL0_SPEC> {
        RAM_ECC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    #[must_use]
    pub fn ram_ecc_inj_en(&mut self) -> RAM_ECC_INJ_EN_W<CM0_CA_CTL0_SPEC> {
        RAM_ECC_INJ_EN_W::new(self, 1)
    }
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    #[must_use]
    pub fn way(&mut self) -> WAY_W<CM0_CA_CTL0_SPEC> {
        WAY_W::new(self, 16)
    }
    #[doc = "Bits 24:26 - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    #[must_use]
    pub fn set_addr(&mut self) -> SET_ADDR_W<CM0_CA_CTL0_SPEC> {
        SET_ADDR_W::new(self, 24)
    }
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PREF_EN_W<CM0_CA_CTL0_SPEC> {
        PREF_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ca_en(&mut self) -> CA_EN_W<CM0_CA_CTL0_SPEC> {
        CA_EN_W::new(self, 31)
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
#[doc = "CM0+ cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm0_ca_ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm0_ca_ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CM0_CA_CTL0_SPEC;
impl crate::RegisterSpec for CM0_CA_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_ca_ctl0::R`](R) reader structure"]
impl crate::Readable for CM0_CA_CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cm0_ca_ctl0::W`](W) writer structure"]
impl crate::Writable for CM0_CA_CTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM0_CA_CTL0 to value 0xc000_0001"]
impl crate::Resettable for CM0_CA_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0001;
}

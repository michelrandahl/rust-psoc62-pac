#[doc = "Register `FAST_CA_CTL` reader"]
pub type R = crate::R<FAST_CA_CTL_SPEC>;
#[doc = "Register `FAST_CA_CTL` writer"]
pub type W = crate::W<FAST_CA_CTL_SPEC>;
#[doc = "Field `WAY` reader - this is for debug purpose only, and should be hidden to customers in technical document"]
pub type WAY_R = crate::FieldReader;
#[doc = "Field `WAY` writer - this is for debug purpose only, and should be hidden to customers in technical document"]
pub type WAY_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SET_ADDR` reader - this is for debug purpose only, and should be hidden to customers in technical document"]
pub type SET_ADDR_R = crate::FieldReader;
#[doc = "Field `SET_ADDR` writer - this is for debug purpose only, and should be hidden to customers in technical document"]
pub type SET_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREF_EN` reader - N/A"]
pub type PREF_EN_R = crate::BitReader;
#[doc = "Field `PREF_EN` writer - N/A"]
pub type PREF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - N/A"]
pub type ENABLED_R = crate::BitReader;
#[doc = "Field `ENABLED` writer - N/A"]
pub type ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - this is for debug purpose only, and should be hidden to customers in technical document"]
    #[inline(always)]
    pub fn way(&self) -> WAY_R {
        WAY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - this is for debug purpose only, and should be hidden to customers in technical document"]
    #[inline(always)]
    pub fn set_addr(&self) -> SET_ADDR_R {
        SET_ADDR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - this is for debug purpose only, and should be hidden to customers in technical document"]
    #[inline(always)]
    #[must_use]
    pub fn way(&mut self) -> WAY_W<FAST_CA_CTL_SPEC> {
        WAY_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - this is for debug purpose only, and should be hidden to customers in technical document"]
    #[inline(always)]
    #[must_use]
    pub fn set_addr(&mut self) -> SET_ADDR_W<FAST_CA_CTL_SPEC> {
        SET_ADDR_W::new(self, 24)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PREF_EN_W<FAST_CA_CTL_SPEC> {
        PREF_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<FAST_CA_CTL_SPEC> {
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
#[doc = "Fast cache control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fast_ca_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fast_ca_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FAST_CA_CTL_SPEC;
impl crate::RegisterSpec for FAST_CA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fast_ca_ctl::R`](R) reader structure"]
impl crate::Readable for FAST_CA_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fast_ca_ctl::W`](W) writer structure"]
impl crate::Writable for FAST_CA_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAST_CA_CTL to value 0xc000_0000"]
impl crate::Resettable for FAST_CA_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}

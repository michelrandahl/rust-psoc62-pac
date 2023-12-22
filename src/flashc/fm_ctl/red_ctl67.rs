#[doc = "Register `RED_CTL67` reader"]
pub type R = crate::R<RED_CTL67_SPEC>;
#[doc = "Register `RED_CTL67` writer"]
pub type W = crate::W<RED_CTL67_SPEC>;
#[doc = "Field `RED_ADDR_6` reader - Bad Row Pair Address for Sector 6"]
pub type RED_ADDR_6_R = crate::FieldReader;
#[doc = "Field `RED_ADDR_6` writer - Bad Row Pair Address for Sector 6"]
pub type RED_ADDR_6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_6` reader - 1: Redundancy Enable for Sector 6"]
pub type RED_EN_6_R = crate::BitReader;
#[doc = "Field `RED_EN_6` writer - 1: Redundancy Enable for Sector 6"]
pub type RED_EN_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_ADDR_7` reader - Bad Row Pair Address for Sector 7"]
pub type RED_ADDR_7_R = crate::FieldReader;
#[doc = "Field `RED_ADDR_7` writer - Bad Row Pair Address for Sector 7"]
pub type RED_ADDR_7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_7` reader - 1: Redundancy Enable for Sector 7"]
pub type RED_EN_7_R = crate::BitReader;
#[doc = "Field `RED_EN_7` writer - 1: Redundancy Enable for Sector 7"]
pub type RED_EN_7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 6"]
    #[inline(always)]
    pub fn red_addr_6(&self) -> RED_ADDR_6_R {
        RED_ADDR_6_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 6"]
    #[inline(always)]
    pub fn red_en_6(&self) -> RED_EN_6_R {
        RED_EN_6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 7"]
    #[inline(always)]
    pub fn red_addr_7(&self) -> RED_ADDR_7_R {
        RED_ADDR_7_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 7"]
    #[inline(always)]
    pub fn red_en_7(&self) -> RED_EN_7_R {
        RED_EN_7_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 6"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_6(&mut self) -> RED_ADDR_6_W<RED_CTL67_SPEC> {
        RED_ADDR_6_W::new(self, 0)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 6"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_6(&mut self) -> RED_EN_6_W<RED_CTL67_SPEC> {
        RED_EN_6_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 7"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_7(&mut self) -> RED_ADDR_7_W<RED_CTL67_SPEC> {
        RED_ADDR_7_W::new(self, 16)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 7"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_7(&mut self) -> RED_EN_7_W<RED_CTL67_SPEC> {
        RED_EN_7_W::new(self, 24)
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
#[doc = "Redundancy Control normal sectors 6,7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`red_ctl67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`red_ctl67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RED_CTL67_SPEC;
impl crate::RegisterSpec for RED_CTL67_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`red_ctl67::R`](R) reader structure"]
impl crate::Readable for RED_CTL67_SPEC {}
#[doc = "`write(|w| ..)` method takes [`red_ctl67::W`](W) writer structure"]
impl crate::Writable for RED_CTL67_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RED_CTL67 to value 0"]
impl crate::Resettable for RED_CTL67_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `RED_CTL01` reader"]
pub type R = crate::R<RED_CTL01_SPEC>;
#[doc = "Register `RED_CTL01` writer"]
pub type W = crate::W<RED_CTL01_SPEC>;
#[doc = "Field `RED_ADDR_0` reader - Bad Row Pair Address for Sector 0"]
pub type RED_ADDR_0_R = crate::FieldReader;
#[doc = "Field `RED_ADDR_0` writer - Bad Row Pair Address for Sector 0"]
pub type RED_ADDR_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_0` reader - 1: Redundancy Enable for Sector 0"]
pub type RED_EN_0_R = crate::BitReader;
#[doc = "Field `RED_EN_0` writer - 1: Redundancy Enable for Sector 0"]
pub type RED_EN_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_ADDR_1` reader - Bad Row Pair Address for Sector 1"]
pub type RED_ADDR_1_R = crate::FieldReader;
#[doc = "Field `RED_ADDR_1` writer - Bad Row Pair Address for Sector 1"]
pub type RED_ADDR_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_1` reader - 1: Redundancy Enable for Sector 1"]
pub type RED_EN_1_R = crate::BitReader;
#[doc = "Field `RED_EN_1` writer - 1: Redundancy Enable for Sector 1"]
pub type RED_EN_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    pub fn red_addr_0(&self) -> RED_ADDR_0_R {
        RED_ADDR_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 0"]
    #[inline(always)]
    pub fn red_en_0(&self) -> RED_EN_0_R {
        RED_EN_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    pub fn red_addr_1(&self) -> RED_ADDR_1_R {
        RED_ADDR_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 1"]
    #[inline(always)]
    pub fn red_en_1(&self) -> RED_EN_1_R {
        RED_EN_1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_0(&mut self) -> RED_ADDR_0_W<RED_CTL01_SPEC> {
        RED_ADDR_0_W::new(self, 0)
    }
    #[doc = "Bit 8 - 1: Redundancy Enable for Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_0(&mut self) -> RED_EN_0_W<RED_CTL01_SPEC> {
        RED_EN_0_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_1(&mut self) -> RED_ADDR_1_W<RED_CTL01_SPEC> {
        RED_ADDR_1_W::new(self, 16)
    }
    #[doc = "Bit 24 - 1: Redundancy Enable for Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_1(&mut self) -> RED_EN_1_W<RED_CTL01_SPEC> {
        RED_EN_1_W::new(self, 24)
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
#[doc = "Redundancy Control normal sectors 0,1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`red_ctl01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`red_ctl01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RED_CTL01_SPEC;
impl crate::RegisterSpec for RED_CTL01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`red_ctl01::R`](R) reader structure"]
impl crate::Readable for RED_CTL01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`red_ctl01::W`](W) writer structure"]
impl crate::Writable for RED_CTL01_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RED_CTL01 to value 0"]
impl crate::Resettable for RED_CTL01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

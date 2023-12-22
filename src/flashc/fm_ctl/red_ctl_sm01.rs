#[doc = "Register `RED_CTL_SM01` reader"]
pub type R = crate::R<RED_CTL_SM01_SPEC>;
#[doc = "Register `RED_CTL_SM01` writer"]
pub type W = crate::W<RED_CTL_SM01_SPEC>;
#[doc = "Field `RED_ADDR_SM0` reader - Bad Row Pair Address for Special Sector 0"]
pub type RED_ADDR_SM0_R = crate::FieldReader;
#[doc = "Field `RED_ADDR_SM0` writer - Bad Row Pair Address for Special Sector 0"]
pub type RED_ADDR_SM0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_SM0` reader - Redundancy Enable for Special Sector 0"]
pub type RED_EN_SM0_R = crate::BitReader;
#[doc = "Field `RED_EN_SM0` writer - Redundancy Enable for Special Sector 0"]
pub type RED_EN_SM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_ADDR_SM1` reader - Bad Row Pair Address for Special Sector 1"]
pub type RED_ADDR_SM1_R = crate::FieldReader;
#[doc = "Field `RED_ADDR_SM1` writer - Bad Row Pair Address for Special Sector 1"]
pub type RED_ADDR_SM1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_EN_SM1` reader - Redundancy Enable for Special Sector 1"]
pub type RED_EN_SM1_R = crate::BitReader;
#[doc = "Field `RED_EN_SM1` writer - Redundancy Enable for Special Sector 1"]
pub type RED_EN_SM1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    pub fn red_addr_sm0(&self) -> RED_ADDR_SM0_R {
        RED_ADDR_SM0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    pub fn red_en_sm0(&self) -> RED_EN_SM0_R {
        RED_EN_SM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    pub fn red_addr_sm1(&self) -> RED_ADDR_SM1_R {
        RED_ADDR_SM1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    pub fn red_en_sm1(&self) -> RED_EN_SM1_R {
        RED_EN_SM1_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_sm0(&mut self) -> RED_ADDR_SM0_W<RED_CTL_SM01_SPEC> {
        RED_ADDR_SM0_W::new(self, 0)
    }
    #[doc = "Bit 8 - Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_sm0(&mut self) -> RED_EN_SM0_W<RED_CTL_SM01_SPEC> {
        RED_EN_SM0_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn red_addr_sm1(&mut self) -> RED_ADDR_SM1_W<RED_CTL_SM01_SPEC> {
        RED_ADDR_SM1_W::new(self, 16)
    }
    #[doc = "Bit 24 - Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    #[must_use]
    pub fn red_en_sm1(&mut self) -> RED_EN_SM1_W<RED_CTL_SM01_SPEC> {
        RED_EN_SM1_W::new(self, 24)
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
#[doc = "Redundancy Control special sectors 0,1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`red_ctl_sm01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`red_ctl_sm01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RED_CTL_SM01_SPEC;
impl crate::RegisterSpec for RED_CTL_SM01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`red_ctl_sm01::R`](R) reader structure"]
impl crate::Readable for RED_CTL_SM01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`red_ctl_sm01::W`](W) writer structure"]
impl crate::Writable for RED_CTL_SM01_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RED_CTL_SM01 to value 0"]
impl crate::Resettable for RED_CTL_SM01_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

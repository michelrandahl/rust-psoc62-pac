#[doc = "Register `EP_ACTIVE` reader"]
pub type R = crate::R<EP_ACTIVE_SPEC>;
#[doc = "Register `EP_ACTIVE` writer"]
pub type W = crate::W<EP_ACTIVE_SPEC>;
#[doc = "Field `EP1_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP1_ACT_R = crate::BitReader;
#[doc = "Field `EP1_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP1_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP2_ACT_R = crate::BitReader;
#[doc = "Field `EP2_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP2_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP3_ACT_R = crate::BitReader;
#[doc = "Field `EP3_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP3_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP4_ACT_R = crate::BitReader;
#[doc = "Field `EP4_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP4_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP5_ACT_R = crate::BitReader;
#[doc = "Field `EP5_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP5_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP6_ACT_R = crate::BitReader;
#[doc = "Field `EP6_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP6_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP7_ACT_R = crate::BitReader;
#[doc = "Field `EP7_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP7_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_ACT` reader - Indicates that Endpoint is currently active."]
pub type EP8_ACT_R = crate::BitReader;
#[doc = "Field `EP8_ACT` writer - Indicates that Endpoint is currently active."]
pub type EP8_ACT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep1_act(&self) -> EP1_ACT_R {
        EP1_ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep2_act(&self) -> EP2_ACT_R {
        EP2_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep3_act(&self) -> EP3_ACT_R {
        EP3_ACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep4_act(&self) -> EP4_ACT_R {
        EP4_ACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep5_act(&self) -> EP5_ACT_R {
        EP5_ACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep6_act(&self) -> EP6_ACT_R {
        EP6_ACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep7_act(&self) -> EP7_ACT_R {
        EP7_ACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep8_act(&self) -> EP8_ACT_R {
        EP8_ACT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep1_act(&mut self) -> EP1_ACT_W<EP_ACTIVE_SPEC> {
        EP1_ACT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep2_act(&mut self) -> EP2_ACT_W<EP_ACTIVE_SPEC> {
        EP2_ACT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep3_act(&mut self) -> EP3_ACT_W<EP_ACTIVE_SPEC> {
        EP3_ACT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep4_act(&mut self) -> EP4_ACT_W<EP_ACTIVE_SPEC> {
        EP4_ACT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep5_act(&mut self) -> EP5_ACT_W<EP_ACTIVE_SPEC> {
        EP5_ACT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep6_act(&mut self) -> EP6_ACT_W<EP_ACTIVE_SPEC> {
        EP6_ACT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep7_act(&mut self) -> EP7_ACT_W<EP_ACTIVE_SPEC> {
        EP7_ACT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep8_act(&mut self) -> EP8_ACT_W<EP_ACTIVE_SPEC> {
        EP8_ACT_W::new(self, 7)
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
#[doc = "Endpoint Active Indication Register *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_active::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_active::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP_ACTIVE_SPEC;
impl crate::RegisterSpec for EP_ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_active::R`](R) reader structure"]
impl crate::Readable for EP_ACTIVE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep_active::W`](W) writer structure"]
impl crate::Writable for EP_ACTIVE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EP_ACTIVE to value 0"]
impl crate::Resettable for EP_ACTIVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

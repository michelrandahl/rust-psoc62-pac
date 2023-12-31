#[doc = "Register `ARB_INT_EN` reader"]
pub type R = crate::R<ARB_INT_EN_SPEC>;
#[doc = "Register `ARB_INT_EN` writer"]
pub type W = crate::W<ARB_INT_EN_SPEC>;
#[doc = "Field `EP1_INTR_EN` reader - Enables interrupt for EP1"]
pub type EP1_INTR_EN_R = crate::BitReader;
#[doc = "Field `EP1_INTR_EN` writer - Enables interrupt for EP1"]
pub type EP1_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_INTR_EN` reader - Enables interrupt for EP2"]
pub type EP2_INTR_EN_R = crate::BitReader;
#[doc = "Field `EP2_INTR_EN` writer - Enables interrupt for EP2"]
pub type EP2_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_INTR_EN` reader - Enables interrupt for EP3"]
pub type EP3_INTR_EN_R = crate::BitReader;
#[doc = "Field `EP3_INTR_EN` writer - Enables interrupt for EP3"]
pub type EP3_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_INTR_EN` reader - Enables interrupt for EP4"]
pub type EP4_INTR_EN_R = crate::BitReader;
#[doc = "Field `EP4_INTR_EN` writer - Enables interrupt for EP4"]
pub type EP4_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_INTR_EN` reader - Enables interrupt for EP5"]
pub type EP5_INTR_EN_R = crate::BitReader;
#[doc = "Field `EP5_INTR_EN` writer - Enables interrupt for EP5"]
pub type EP5_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_INTR_EN` reader - Enables interrupt for EP6"]
pub type EP6_INTR_EN_R = crate::BitReader;
#[doc = "Field `EP6_INTR_EN` writer - Enables interrupt for EP6"]
pub type EP6_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_INTR_EN` reader - Enables interrupt for EP7"]
pub type EP7_INTR_EN_R = crate::BitReader;
#[doc = "Field `EP7_INTR_EN` writer - Enables interrupt for EP7"]
pub type EP7_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_INTR_EN` reader - Enables interrupt for EP8"]
pub type EP8_INTR_EN_R = crate::BitReader;
#[doc = "Field `EP8_INTR_EN` writer - Enables interrupt for EP8"]
pub type EP8_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables interrupt for EP1"]
    #[inline(always)]
    pub fn ep1_intr_en(&self) -> EP1_INTR_EN_R {
        EP1_INTR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables interrupt for EP2"]
    #[inline(always)]
    pub fn ep2_intr_en(&self) -> EP2_INTR_EN_R {
        EP2_INTR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables interrupt for EP3"]
    #[inline(always)]
    pub fn ep3_intr_en(&self) -> EP3_INTR_EN_R {
        EP3_INTR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables interrupt for EP4"]
    #[inline(always)]
    pub fn ep4_intr_en(&self) -> EP4_INTR_EN_R {
        EP4_INTR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables interrupt for EP5"]
    #[inline(always)]
    pub fn ep5_intr_en(&self) -> EP5_INTR_EN_R {
        EP5_INTR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables interrupt for EP6"]
    #[inline(always)]
    pub fn ep6_intr_en(&self) -> EP6_INTR_EN_R {
        EP6_INTR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables interrupt for EP7"]
    #[inline(always)]
    pub fn ep7_intr_en(&self) -> EP7_INTR_EN_R {
        EP7_INTR_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables interrupt for EP8"]
    #[inline(always)]
    pub fn ep8_intr_en(&self) -> EP8_INTR_EN_R {
        EP8_INTR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables interrupt for EP1"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_intr_en(&mut self) -> EP1_INTR_EN_W<ARB_INT_EN_SPEC> {
        EP1_INTR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables interrupt for EP2"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_intr_en(&mut self) -> EP2_INTR_EN_W<ARB_INT_EN_SPEC> {
        EP2_INTR_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables interrupt for EP3"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_intr_en(&mut self) -> EP3_INTR_EN_W<ARB_INT_EN_SPEC> {
        EP3_INTR_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables interrupt for EP4"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_intr_en(&mut self) -> EP4_INTR_EN_W<ARB_INT_EN_SPEC> {
        EP4_INTR_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables interrupt for EP5"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_intr_en(&mut self) -> EP5_INTR_EN_W<ARB_INT_EN_SPEC> {
        EP5_INTR_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enables interrupt for EP6"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_intr_en(&mut self) -> EP6_INTR_EN_W<ARB_INT_EN_SPEC> {
        EP6_INTR_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enables interrupt for EP7"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_intr_en(&mut self) -> EP7_INTR_EN_W<ARB_INT_EN_SPEC> {
        EP7_INTR_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enables interrupt for EP8"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_intr_en(&mut self) -> EP8_INTR_EN_W<ARB_INT_EN_SPEC> {
        EP8_INTR_EN_W::new(self, 7)
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
#[doc = "Arbiter Interrupt Enable *1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARB_INT_EN_SPEC;
impl crate::RegisterSpec for ARB_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_int_en::R`](R) reader structure"]
impl crate::Readable for ARB_INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arb_int_en::W`](W) writer structure"]
impl crate::Writable for ARB_INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARB_INT_EN to value 0"]
impl crate::Resettable for ARB_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

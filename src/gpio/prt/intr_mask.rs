#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<INTR_MASK_SPEC>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<INTR_MASK_SPEC>;
#[doc = "Field `EDGE0` reader - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
pub type EDGE0_R = crate::BitReader;
#[doc = "Field `EDGE0` writer - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
pub type EDGE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE1` reader - Masks edge interrupt on IO pin 1"]
pub type EDGE1_R = crate::BitReader;
#[doc = "Field `EDGE1` writer - Masks edge interrupt on IO pin 1"]
pub type EDGE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE2` reader - Masks edge interrupt on IO pin 2"]
pub type EDGE2_R = crate::BitReader;
#[doc = "Field `EDGE2` writer - Masks edge interrupt on IO pin 2"]
pub type EDGE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE3` reader - Masks edge interrupt on IO pin 3"]
pub type EDGE3_R = crate::BitReader;
#[doc = "Field `EDGE3` writer - Masks edge interrupt on IO pin 3"]
pub type EDGE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE4` reader - Masks edge interrupt on IO pin 4"]
pub type EDGE4_R = crate::BitReader;
#[doc = "Field `EDGE4` writer - Masks edge interrupt on IO pin 4"]
pub type EDGE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE5` reader - Masks edge interrupt on IO pin 5"]
pub type EDGE5_R = crate::BitReader;
#[doc = "Field `EDGE5` writer - Masks edge interrupt on IO pin 5"]
pub type EDGE5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE6` reader - Masks edge interrupt on IO pin 6"]
pub type EDGE6_R = crate::BitReader;
#[doc = "Field `EDGE6` writer - Masks edge interrupt on IO pin 6"]
pub type EDGE6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE7` reader - Masks edge interrupt on IO pin 7"]
pub type EDGE7_R = crate::BitReader;
#[doc = "Field `EDGE7` writer - Masks edge interrupt on IO pin 7"]
pub type EDGE7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT_EDGE` reader - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FLT_EDGE_R = crate::BitReader;
#[doc = "Field `FLT_EDGE` writer - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FLT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masks edge interrupt on IO pin 1"]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masks edge interrupt on IO pin 2"]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masks edge interrupt on IO pin 3"]
    #[inline(always)]
    pub fn edge3(&self) -> EDGE3_R {
        EDGE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masks edge interrupt on IO pin 4"]
    #[inline(always)]
    pub fn edge4(&self) -> EDGE4_R {
        EDGE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masks edge interrupt on IO pin 5"]
    #[inline(always)]
    pub fn edge5(&self) -> EDGE5_R {
        EDGE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masks edge interrupt on IO pin 6"]
    #[inline(always)]
    pub fn edge6(&self) -> EDGE6_R {
        EDGE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masks edge interrupt on IO pin 7"]
    #[inline(always)]
    pub fn edge7(&self) -> EDGE7_R {
        EDGE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&self) -> FLT_EDGE_R {
        FLT_EDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
    #[inline(always)]
    #[must_use]
    pub fn edge0(&mut self) -> EDGE0_W<INTR_MASK_SPEC> {
        EDGE0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Masks edge interrupt on IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn edge1(&mut self) -> EDGE1_W<INTR_MASK_SPEC> {
        EDGE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Masks edge interrupt on IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn edge2(&mut self) -> EDGE2_W<INTR_MASK_SPEC> {
        EDGE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Masks edge interrupt on IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn edge3(&mut self) -> EDGE3_W<INTR_MASK_SPEC> {
        EDGE3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Masks edge interrupt on IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn edge4(&mut self) -> EDGE4_W<INTR_MASK_SPEC> {
        EDGE4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Masks edge interrupt on IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn edge5(&mut self) -> EDGE5_W<INTR_MASK_SPEC> {
        EDGE5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Masks edge interrupt on IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn edge6(&mut self) -> EDGE6_W<INTR_MASK_SPEC> {
        EDGE6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Masks edge interrupt on IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn edge7(&mut self) -> EDGE7_W<INTR_MASK_SPEC> {
        EDGE7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn flt_edge(&mut self) -> FLT_EDGE_W<INTR_MASK_SPEC> {
        FLT_EDGE_W::new(self, 8)
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
#[doc = "Port interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

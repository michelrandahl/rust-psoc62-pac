#[doc = "Register `SW_REFGEN_SEL` reader"]
pub type R = crate::R<SW_REFGEN_SEL_SPEC>;
#[doc = "Register `SW_REFGEN_SEL` writer"]
pub type W = crate::W<SW_REFGEN_SEL_SPEC>;
#[doc = "Field `SW_IAIB` reader - Set corresponding switch"]
pub type SW_IAIB_R = crate::BitReader;
#[doc = "Field `SW_IAIB` writer - Set corresponding switch"]
pub type SW_IAIB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IBCB` reader - Set corresponding switch"]
pub type SW_IBCB_R = crate::BitReader;
#[doc = "Field `SW_IBCB` writer - Set corresponding switch"]
pub type SW_IBCB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SGMB` reader - Set corresponding switch"]
pub type SW_SGMB_R = crate::BitReader;
#[doc = "Field `SW_SGMB` writer - Set corresponding switch"]
pub type SW_SGMB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SGRP` reader - Set corresponding switch"]
pub type SW_SGRP_R = crate::BitReader;
#[doc = "Field `SW_SGRP` writer - Set corresponding switch"]
pub type SW_SGRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SGRE` reader - Set corresponding switch"]
pub type SW_SGRE_R = crate::BitReader;
#[doc = "Field `SW_SGRE` writer - Set corresponding switch"]
pub type SW_SGRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SGR` reader - Set corresponding switch"]
pub type SW_SGR_R = crate::BitReader;
#[doc = "Field `SW_SGR` writer - Set corresponding switch"]
pub type SW_SGR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_iaib(&self) -> SW_IAIB_R {
        SW_IAIB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ibcb(&self) -> SW_IBCB_R {
        SW_IBCB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgmb(&self) -> SW_SGMB_R {
        SW_SGMB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgrp(&self) -> SW_SGRP_R {
        SW_SGRP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgre(&self) -> SW_SGRE_R {
        SW_SGRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgr(&self) -> SW_SGR_R {
        SW_SGR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_iaib(&mut self) -> SW_IAIB_W<SW_REFGEN_SEL_SPEC> {
        SW_IAIB_W::new(self, 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ibcb(&mut self) -> SW_IBCB_W<SW_REFGEN_SEL_SPEC> {
        SW_IBCB_W::new(self, 4)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sgmb(&mut self) -> SW_SGMB_W<SW_REFGEN_SEL_SPEC> {
        SW_SGMB_W::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sgrp(&mut self) -> SW_SGRP_W<SW_REFGEN_SEL_SPEC> {
        SW_SGRP_W::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sgre(&mut self) -> SW_SGRE_W<SW_REFGEN_SEL_SPEC> {
        SW_SGRE_W::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sgr(&mut self) -> SW_SGR_W<SW_REFGEN_SEL_SPEC> {
        SW_SGR_W::new(self, 28)
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
#[doc = "Reference Generator Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_refgen_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_refgen_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_REFGEN_SEL_SPEC;
impl crate::RegisterSpec for SW_REFGEN_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_refgen_sel::R`](R) reader structure"]
impl crate::Readable for SW_REFGEN_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_refgen_sel::W`](W) writer structure"]
impl crate::Writable for SW_REFGEN_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_REFGEN_SEL to value 0"]
impl crate::Resettable for SW_REFGEN_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

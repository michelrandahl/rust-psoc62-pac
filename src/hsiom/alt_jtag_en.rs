#[doc = "Register `ALT_JTAG_EN` reader"]
pub type R = crate::R<ALT_JTAG_EN_SPEC>;
#[doc = "Register `ALT_JTAG_EN` writer"]
pub type W = crate::W<ALT_JTAG_EN_SPEC>;
#[doc = "Field `ENABLE` reader - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<ALT_JTAG_EN_SPEC> {
        ENABLE_W::new(self, 31)
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
#[doc = "Alternate JTAG IF selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alt_jtag_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alt_jtag_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALT_JTAG_EN_SPEC;
impl crate::RegisterSpec for ALT_JTAG_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alt_jtag_en::R`](R) reader structure"]
impl crate::Readable for ALT_JTAG_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alt_jtag_en::W`](W) writer structure"]
impl crate::Writable for ALT_JTAG_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALT_JTAG_EN to value 0"]
impl crate::Resettable for ALT_JTAG_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

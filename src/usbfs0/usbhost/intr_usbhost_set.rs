#[doc = "Register `INTR_USBHOST_SET` reader"]
pub type R = crate::R<INTR_USBHOST_SET_SPEC>;
#[doc = "Register `INTR_USBHOST_SET` writer"]
pub type W = crate::W<INTR_USBHOST_SET_SPEC>;
#[doc = "Field `SOFIRQS` reader - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type SOFIRQS_R = crate::BitReader;
#[doc = "Field `SOFIRQS` writer - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type SOFIRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRQS` reader - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type DIRQS_R = crate::BitReader;
#[doc = "Field `DIRQS` writer - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type DIRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNNIRQS` reader - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CNNIRQS_R = crate::BitReader;
#[doc = "Field `CNNIRQS` writer - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CNNIRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIRQS` reader - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CMPIRQS_R = crate::BitReader;
#[doc = "Field `CMPIRQS` writer - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type CMPIRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URIRQS` reader - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type URIRQS_R = crate::BitReader;
#[doc = "Field `URIRQS` writer - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type URIRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKIRQS` reader - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type RWKIRQS_R = crate::BitReader;
#[doc = "Field `RWKIRQS` writer - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type RWKIRQS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_6` reader - BCNFTEST interrupt. This bit is test bit"]
pub type RSVD_6_R = crate::BitReader;
#[doc = "Field `RSVD_6` writer - BCNFTEST interrupt. This bit is test bit"]
pub type RSVD_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCANS` reader - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type TCANS_R = crate::BitReader;
#[doc = "Field `TCANS` writer - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
pub type TCANS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn sofirqs(&self) -> SOFIRQS_R {
        SOFIRQS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn dirqs(&self) -> DIRQS_R {
        DIRQS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cnnirqs(&self) -> CNNIRQS_R {
        CNNIRQS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn cmpirqs(&self) -> CMPIRQS_R {
        CMPIRQS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn urirqs(&self) -> URIRQS_R {
        URIRQS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn rwkirqs(&self) -> RWKIRQS_R {
        RWKIRQS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BCNFTEST interrupt. This bit is test bit"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    pub fn tcans(&self) -> TCANS_R {
        TCANS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets SOFIRQ bit. If this bit is written to '1', SOFIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn sofirqs(&mut self) -> SOFIRQS_W<INTR_USBHOST_SET_SPEC> {
        SOFIRQS_W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit sets DIRQ bit. If this bit is written to '1', DIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn dirqs(&mut self) -> DIRQS_W<INTR_USBHOST_SET_SPEC> {
        DIRQS_W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit sets CNNIRQ bit. If this bit is written to '1', CNNIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn cnnirqs(&mut self) -> CNNIRQS_W<INTR_USBHOST_SET_SPEC> {
        CNNIRQS_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit sets CMPIRQ bit. If this bit is written to '1', CMPIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn cmpirqs(&mut self) -> CMPIRQS_W<INTR_USBHOST_SET_SPEC> {
        CMPIRQS_W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit sets URIRQ bit. If this bit is written to '1', URIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn urirqs(&mut self) -> URIRQS_W<INTR_USBHOST_SET_SPEC> {
        URIRQS_W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit sets RWKIRQ bit. If this bit is written to '1', RWKIRQ is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn rwkirqs(&mut self) -> RWKIRQS_W<INTR_USBHOST_SET_SPEC> {
        RWKIRQS_W::new(self, 5)
    }
    #[doc = "Bit 6 - BCNFTEST interrupt. This bit is test bit"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_6(&mut self) -> RSVD_6_W<INTR_USBHOST_SET_SPEC> {
        RSVD_6_W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit sets TCAN bit. If this bit is written to '1', TCAN is set to '1'. However, if this bit is written with '0', its value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn tcans(&mut self) -> TCANS_W<INTR_USBHOST_SET_SPEC> {
        TCANS_W::new(self, 7)
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
#[doc = "Interrupt USB Host Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_usbhost_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_usbhost_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_USBHOST_SET_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_usbhost_set::R`](R) reader structure"]
impl crate::Readable for INTR_USBHOST_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_usbhost_set::W`](W) writer structure"]
impl crate::Writable for INTR_USBHOST_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_USBHOST_SET to value 0"]
impl crate::Resettable for INTR_USBHOST_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `PWR_TRIM_BODOVP_CTL` reader"]
pub type R = crate::R<PWR_TRIM_BODOVP_CTL_SPEC>;
#[doc = "Register `PWR_TRIM_BODOVP_CTL` writer"]
pub type W = crate::W<PWR_TRIM_BODOVP_CTL_SPEC>;
#[doc = "Field `HVPORBOD_TRIPSEL` reader - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_TRIPSEL_R = crate::FieldReader;
#[doc = "Field `HVPORBOD_TRIPSEL` writer - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_TRIPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HVPORBOD_OFSTRIM` reader - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_OFSTRIM_R = crate::FieldReader;
#[doc = "Field `HVPORBOD_OFSTRIM` writer - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_OFSTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HVPORBOD_ITRIM` reader - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_ITRIM_R = crate::FieldReader;
#[doc = "Field `HVPORBOD_ITRIM` writer - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_ITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LVPORBOD_TRIPSEL` reader - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_TRIPSEL_R = crate::FieldReader;
#[doc = "Field `LVPORBOD_TRIPSEL` writer - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_TRIPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LVPORBOD_OFSTRIM` reader - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_OFSTRIM_R = crate::FieldReader;
#[doc = "Field `LVPORBOD_OFSTRIM` writer - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_OFSTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LVPORBOD_ITRIM` reader - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_ITRIM_R = crate::FieldReader;
#[doc = "Field `LVPORBOD_ITRIM` writer - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_ITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_tripsel(&self) -> HVPORBOD_TRIPSEL_R {
        HVPORBOD_TRIPSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_ofstrim(&self) -> HVPORBOD_OFSTRIM_R {
        HVPORBOD_OFSTRIM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_itrim(&self) -> HVPORBOD_ITRIM_R {
        HVPORBOD_ITRIM_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_tripsel(&self) -> LVPORBOD_TRIPSEL_R {
        LVPORBOD_TRIPSEL_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 14:16 - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_ofstrim(&self) -> LVPORBOD_OFSTRIM_R {
        LVPORBOD_OFSTRIM_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_itrim(&self) -> LVPORBOD_ITRIM_R {
        LVPORBOD_ITRIM_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn hvporbod_tripsel(&mut self) -> HVPORBOD_TRIPSEL_W<PWR_TRIM_BODOVP_CTL_SPEC> {
        HVPORBOD_TRIPSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn hvporbod_ofstrim(&mut self) -> HVPORBOD_OFSTRIM_W<PWR_TRIM_BODOVP_CTL_SPEC> {
        HVPORBOD_OFSTRIM_W::new(self, 4)
    }
    #[doc = "Bits 7:9 - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn hvporbod_itrim(&mut self) -> HVPORBOD_ITRIM_W<PWR_TRIM_BODOVP_CTL_SPEC> {
        HVPORBOD_ITRIM_W::new(self, 7)
    }
    #[doc = "Bits 10:12 - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn lvporbod_tripsel(&mut self) -> LVPORBOD_TRIPSEL_W<PWR_TRIM_BODOVP_CTL_SPEC> {
        LVPORBOD_TRIPSEL_W::new(self, 10)
    }
    #[doc = "Bits 14:16 - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn lvporbod_ofstrim(&mut self) -> LVPORBOD_OFSTRIM_W<PWR_TRIM_BODOVP_CTL_SPEC> {
        LVPORBOD_OFSTRIM_W::new(self, 14)
    }
    #[doc = "Bits 17:19 - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn lvporbod_itrim(&mut self) -> LVPORBOD_ITRIM_W<PWR_TRIM_BODOVP_CTL_SPEC> {
        LVPORBOD_ITRIM_W::new(self, 17)
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
#[doc = "BOD/OVP Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_trim_bodovp_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_trim_bodovp_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_TRIM_BODOVP_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_BODOVP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_trim_bodovp_ctl::R`](R) reader structure"]
impl crate::Readable for PWR_TRIM_BODOVP_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_trim_bodovp_ctl::W`](W) writer structure"]
impl crate::Writable for PWR_TRIM_BODOVP_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_TRIM_BODOVP_CTL to value 0x0004_0d04"]
impl crate::Resettable for PWR_TRIM_BODOVP_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0d04;
}

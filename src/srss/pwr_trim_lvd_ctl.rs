#[doc = "Register `PWR_TRIM_LVD_CTL` reader"]
pub type R = crate::R<PWR_TRIM_LVD_CTL_SPEC>;
#[doc = "Register `PWR_TRIM_LVD_CTL` writer"]
pub type W = crate::W<PWR_TRIM_LVD_CTL_SPEC>;
#[doc = "Field `HVLVD1_OFSTRIM` reader - HVLVD1 offset trim"]
pub type HVLVD1_OFSTRIM_R = crate::FieldReader;
#[doc = "Field `HVLVD1_OFSTRIM` writer - HVLVD1 offset trim"]
pub type HVLVD1_OFSTRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HVLVD1_ITRIM` reader - HVLVD1 current trim"]
pub type HVLVD1_ITRIM_R = crate::FieldReader;
#[doc = "Field `HVLVD1_ITRIM` writer - HVLVD1 current trim"]
pub type HVLVD1_ITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - HVLVD1 offset trim"]
    #[inline(always)]
    pub fn hvlvd1_ofstrim(&self) -> HVLVD1_OFSTRIM_R {
        HVLVD1_OFSTRIM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - HVLVD1 current trim"]
    #[inline(always)]
    pub fn hvlvd1_itrim(&self) -> HVLVD1_ITRIM_R {
        HVLVD1_ITRIM_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HVLVD1 offset trim"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1_ofstrim(&mut self) -> HVLVD1_OFSTRIM_W<PWR_TRIM_LVD_CTL_SPEC> {
        HVLVD1_OFSTRIM_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - HVLVD1 current trim"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1_itrim(&mut self) -> HVLVD1_ITRIM_W<PWR_TRIM_LVD_CTL_SPEC> {
        HVLVD1_ITRIM_W::new(self, 4)
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
#[doc = "LVD Trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_trim_lvd_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_trim_lvd_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_TRIM_LVD_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_LVD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_trim_lvd_ctl::R`](R) reader structure"]
impl crate::Readable for PWR_TRIM_LVD_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_trim_lvd_ctl::W`](W) writer structure"]
impl crate::Writable for PWR_TRIM_LVD_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_TRIM_LVD_CTL to value 0x20"]
impl crate::Resettable for PWR_TRIM_LVD_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}

#[doc = "Register `VDD_INTR` reader"]
pub type R = crate::R<VDD_INTR_SPEC>;
#[doc = "Register `VDD_INTR` writer"]
pub type W = crate::W<VDD_INTR_SPEC>;
#[doc = "Field `VDDIO_ACTIVE` reader - Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
pub type VDDIO_ACTIVE_R = crate::FieldReader<u16>;
#[doc = "Field `VDDIO_ACTIVE` writer - Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
pub type VDDIO_ACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VDDA_ACTIVE` reader - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VDDA_ACTIVE_R = crate::BitReader;
#[doc = "Field `VDDA_ACTIVE` writer - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VDDA_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDD_ACTIVE` reader - The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
pub type VDDD_ACTIVE_R = crate::BitReader;
#[doc = "Field `VDDD_ACTIVE` writer - The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
pub type VDDD_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
    #[inline(always)]
    pub fn vddio_active(&self) -> VDDIO_ACTIVE_R {
        VDDIO_ACTIVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&self) -> VDDA_ACTIVE_R {
        VDDA_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
    #[inline(always)]
    pub fn vddd_active(&self) -> VDDD_ACTIVE_R {
        VDDD_ACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_active(&mut self) -> VDDIO_ACTIVE_W<VDD_INTR_SPEC> {
        VDDIO_ACTIVE_W::new(self, 0)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    #[must_use]
    pub fn vdda_active(&mut self) -> VDDA_ACTIVE_W<VDD_INTR_SPEC> {
        VDDA_ACTIVE_W::new(self, 30)
    }
    #[doc = "Bit 31 - The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
    #[inline(always)]
    #[must_use]
    pub fn vddd_active(&mut self) -> VDDD_ACTIVE_W<VDD_INTR_SPEC> {
        VDDD_ACTIVE_W::new(self, 31)
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
#[doc = "Supply detection interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdd_intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDD_INTR_SPEC;
impl crate::RegisterSpec for VDD_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_intr::R`](R) reader structure"]
impl crate::Readable for VDD_INTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdd_intr::W`](W) writer structure"]
impl crate::Writable for VDD_INTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDD_INTR to value 0"]
impl crate::Resettable for VDD_INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

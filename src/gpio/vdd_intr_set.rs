#[doc = "Register `VDD_INTR_SET` reader"]
pub type R = crate::R<VDD_INTR_SET_SPEC>;
#[doc = "Register `VDD_INTR_SET` writer"]
pub type W = crate::W<VDD_INTR_SET_SPEC>;
#[doc = "Field `VDDIO_ACTIVE` reader - Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
pub type VDDIO_ACTIVE_R = crate::FieldReader<u16>;
#[doc = "Field `VDDIO_ACTIVE` writer - Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
pub type VDDIO_ACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VDDA_ACTIVE` reader - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VDDA_ACTIVE_R = crate::BitReader;
#[doc = "Field `VDDA_ACTIVE` writer - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VDDA_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDD_ACTIVE` reader - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub type VDDD_ACTIVE_R = crate::BitReader;
#[doc = "Field `VDDD_ACTIVE` writer - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub type VDDD_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    pub fn vddio_active(&self) -> VDDIO_ACTIVE_R {
        VDDIO_ACTIVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&self) -> VDDA_ACTIVE_R {
        VDDA_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn vddd_active(&self) -> VDDD_ACTIVE_R {
        VDDD_ACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_active(&mut self) -> VDDIO_ACTIVE_W<VDD_INTR_SET_SPEC> {
        VDDIO_ACTIVE_W::new(self, 0)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    #[must_use]
    pub fn vdda_active(&mut self) -> VDDA_ACTIVE_W<VDD_INTR_SET_SPEC> {
        VDDA_ACTIVE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    #[must_use]
    pub fn vddd_active(&mut self) -> VDDD_ACTIVE_W<VDD_INTR_SET_SPEC> {
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
#[doc = "Supply detection interrupt set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdd_intr_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdd_intr_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDD_INTR_SET_SPEC;
impl crate::RegisterSpec for VDD_INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_intr_set::R`](R) reader structure"]
impl crate::Readable for VDD_INTR_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdd_intr_set::W`](W) writer structure"]
impl crate::Writable for VDD_INTR_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDD_INTR_SET to value 0"]
impl crate::Resettable for VDD_INTR_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

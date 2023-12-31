#[doc = "Register `WR_DUMMY_CTL` reader"]
pub type R = crate::R<WR_DUMMY_CTL_SPEC>;
#[doc = "Register `WR_DUMMY_CTL` writer"]
pub type W = crate::W<WR_DUMMY_CTL_SPEC>;
#[doc = "Field `SIZE5` reader - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
pub type SIZE5_R = crate::FieldReader;
#[doc = "Field `SIZE5` writer - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
pub type SIZE5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRESENT` reader - Presence of dummy cycles: '0': not present '1': present"]
pub type PRESENT_R = crate::BitReader;
#[doc = "Field `PRESENT` writer - Presence of dummy cycles: '0': not present '1': present"]
pub type PRESENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
    #[inline(always)]
    pub fn size5(&self) -> SIZE5_R {
        SIZE5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn size5(&mut self) -> SIZE5_W<WR_DUMMY_CTL_SPEC> {
        SIZE5_W::new(self, 0)
    }
    #[doc = "Bit 31 - Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PRESENT_W<WR_DUMMY_CTL_SPEC> {
        PRESENT_W::new(self, 31)
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
#[doc = "Write dummy control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_dummy_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_dummy_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_DUMMY_CTL_SPEC;
impl crate::RegisterSpec for WR_DUMMY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_dummy_ctl::R`](R) reader structure"]
impl crate::Readable for WR_DUMMY_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_dummy_ctl::W`](W) writer structure"]
impl crate::Writable for WR_DUMMY_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_DUMMY_CTL to value 0"]
impl crate::Resettable for WR_DUMMY_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

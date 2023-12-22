#[doc = "Register `RES_CAUSE2` reader"]
pub type R = crate::R<RES_CAUSE2_SPEC>;
#[doc = "Register `RES_CAUSE2` writer"]
pub type W = crate::W<RES_CAUSE2_SPEC>;
#[doc = "Field `RESET_CSV_HF_LOSS` reader - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_LOSS_R = crate::FieldReader<u16>;
#[doc = "Field `RESET_CSV_HF_LOSS` writer - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_LOSS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESET_CSV_HF_FREQ` reader - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_FREQ_R = crate::FieldReader<u16>;
#[doc = "Field `RESET_CSV_HF_FREQ` writer - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type RESET_CSV_HF_FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_loss(&self) -> RESET_CSV_HF_LOSS_R {
        RESET_CSV_HF_LOSS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_freq(&self) -> RESET_CSV_HF_FREQ_R {
        RESET_CSV_HF_FREQ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    #[must_use]
    pub fn reset_csv_hf_loss(&mut self) -> RESET_CSV_HF_LOSS_W<RES_CAUSE2_SPEC> {
        RESET_CSV_HF_LOSS_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    #[must_use]
    pub fn reset_csv_hf_freq(&mut self) -> RESET_CSV_HF_FREQ_W<RES_CAUSE2_SPEC> {
        RESET_CSV_HF_FREQ_W::new(self, 16)
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
#[doc = "Reset Cause Observation Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res_cause2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res_cause2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RES_CAUSE2_SPEC;
impl crate::RegisterSpec for RES_CAUSE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res_cause2::R`](R) reader structure"]
impl crate::Readable for RES_CAUSE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`res_cause2::W`](W) writer structure"]
impl crate::Writable for RES_CAUSE2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RES_CAUSE2 to value 0"]
impl crate::Resettable for RES_CAUSE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

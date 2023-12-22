#[doc = "Register `SEQ_NORM_CNT` reader"]
pub type R = crate::R<SEQ_NORM_CNT_SPEC>;
#[doc = "Register `SEQ_NORM_CNT` writer"]
pub type W = crate::W<SEQ_NORM_CNT_SPEC>;
#[doc = "Field `CONV_CNT` reader - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
pub type CONV_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CONV_CNT` writer - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
pub type CONV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    pub fn conv_cnt(&self) -> CONV_CNT_R {
        CONV_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    #[must_use]
    pub fn conv_cnt(&mut self) -> CONV_CNT_W<SEQ_NORM_CNT_SPEC> {
        CONV_CNT_W::new(self, 0)
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
#[doc = "Sequencer Normal conversion and sample counts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_norm_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_norm_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_NORM_CNT_SPEC;
impl crate::RegisterSpec for SEQ_NORM_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_norm_cnt::R`](R) reader structure"]
impl crate::Readable for SEQ_NORM_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_norm_cnt::W`](W) writer structure"]
impl crate::Writable for SEQ_NORM_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ_NORM_CNT to value 0"]
impl crate::Resettable for SEQ_NORM_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

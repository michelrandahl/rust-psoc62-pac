#[doc = "Register `SEQ_INIT_CNT` reader"]
pub type R = crate::R<SEQ_INIT_CNT_SPEC>;
#[doc = "Register `SEQ_INIT_CNT` writer"]
pub type W = crate::W<SEQ_INIT_CNT_SPEC>;
#[doc = "Field `CONV_CNT` reader - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
pub type CONV_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CONV_CNT` writer - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
pub type CONV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    pub fn conv_cnt(&self) -> CONV_CNT_R {
        CONV_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    #[must_use]
    pub fn conv_cnt(&mut self) -> CONV_CNT_W<SEQ_INIT_CNT_SPEC> {
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
#[doc = "Sequencer Initial conversion and sample counts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_init_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_init_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_INIT_CNT_SPEC;
impl crate::RegisterSpec for SEQ_INIT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_init_cnt::R`](R) reader structure"]
impl crate::Readable for SEQ_INIT_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_init_cnt::W`](W) writer structure"]
impl crate::Writable for SEQ_INIT_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ_INIT_CNT to value 0"]
impl crate::Resettable for SEQ_INIT_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

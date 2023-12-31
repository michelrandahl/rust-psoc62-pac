#[doc = "Register `SEQ_START` reader"]
pub type R = crate::R<SEQ_START_SPEC>;
#[doc = "Register `SEQ_START` writer"]
pub type W = crate::W<SEQ_START_SPEC>;
#[doc = "Field `START` reader - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_MODE` reader - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
pub type SEQ_MODE_R = crate::BitReader;
#[doc = "Field `SEQ_MODE` writer - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
pub type SEQ_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
pub type ABORT_R = crate::BitReader;
#[doc = "Field `ABORT` writer - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_START_EN` reader - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
pub type DSI_START_EN_R = crate::BitReader;
#[doc = "Field `DSI_START_EN` writer - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
pub type DSI_START_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AZ0_SKIP` reader - When set the AutoZero_0 state will be skipped"]
pub type AZ0_SKIP_R = crate::BitReader;
#[doc = "Field `AZ0_SKIP` writer - When set the AutoZero_0 state will be skipped"]
pub type AZ0_SKIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AZ1_SKIP` reader - When set the AutoZero_1 state will be skipped"]
pub type AZ1_SKIP_R = crate::BitReader;
#[doc = "Field `AZ1_SKIP` writer - When set the AutoZero_1 state will be skipped"]
pub type AZ1_SKIP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub fn seq_mode(&self) -> SEQ_MODE_R {
        SEQ_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub fn dsi_start_en(&self) -> DSI_START_EN_R {
        DSI_START_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub fn az0_skip(&self) -> AZ0_SKIP_R {
        AZ0_SKIP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub fn az1_skip(&self) -> AZ1_SKIP_R {
        AZ1_SKIP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<SEQ_START_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    #[must_use]
    pub fn seq_mode(&mut self) -> SEQ_MODE_W<SEQ_START_SPEC> {
        SEQ_MODE_W::new(self, 1)
    }
    #[doc = "Bit 3 - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<SEQ_START_SPEC> {
        ABORT_W::new(self, 3)
    }
    #[doc = "Bit 4 - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_start_en(&mut self) -> DSI_START_EN_W<SEQ_START_SPEC> {
        DSI_START_EN_W::new(self, 4)
    }
    #[doc = "Bit 8 - When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    #[must_use]
    pub fn az0_skip(&mut self) -> AZ0_SKIP_W<SEQ_START_SPEC> {
        AZ0_SKIP_W::new(self, 8)
    }
    #[doc = "Bit 9 - When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    #[must_use]
    pub fn az1_skip(&mut self) -> AZ1_SKIP_W<SEQ_START_SPEC> {
        AZ1_SKIP_W::new(self, 9)
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
#[doc = "Sequencer start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_START_SPEC;
impl crate::RegisterSpec for SEQ_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_start::R`](R) reader structure"]
impl crate::Readable for SEQ_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_start::W`](W) writer structure"]
impl crate::Writable for SEQ_START_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ_START to value 0"]
impl crate::Resettable for SEQ_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

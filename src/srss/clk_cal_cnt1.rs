#[doc = "Register `CLK_CAL_CNT1` reader"]
pub type R = crate::R<CLK_CAL_CNT1_SPEC>;
#[doc = "Register `CLK_CAL_CNT1` writer"]
pub type W = crate::W<CLK_CAL_CNT1_SPEC>;
#[doc = "Field `CAL_COUNTER1` reader - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
pub type CAL_COUNTER1_R = crate::FieldReader<u32>;
#[doc = "Field `CAL_COUNTER1` writer - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
pub type CAL_COUNTER1_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `CAL_COUNTER_DONE` reader - Status bit indicating that the internal counter #1 is finished counting and CLK_CAL_CNT2.COUNTER stopped counting up"]
pub type CAL_COUNTER_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
    #[inline(always)]
    pub fn cal_counter1(&self) -> CAL_COUNTER1_R {
        CAL_COUNTER1_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Status bit indicating that the internal counter #1 is finished counting and CLK_CAL_CNT2.COUNTER stopped counting up"]
    #[inline(always)]
    pub fn cal_counter_done(&self) -> CAL_COUNTER_DONE_R {
        CAL_COUNTER_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Down-counter clocked on fast clock output #0 (see CLK_OUTPUT_FAST). This register always reads as zero. Counting starts internally when this register is written with a nonzero value. CAL_COUNTER_DONE goes immediately low to indicate that the counter has started and will be asserted when the counters are done. Do not write this field unless CAL_COUNTER_DONE==1. Both clocks must be running or the measurement will not complete. A stalled counter can be recovered by selecting valid clocks, waiting until the measurement completes, and discarding the first result."]
    #[inline(always)]
    #[must_use]
    pub fn cal_counter1(&mut self) -> CAL_COUNTER1_W<CLK_CAL_CNT1_SPEC> {
        CAL_COUNTER1_W::new(self, 0)
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
#[doc = "Clock Calibration Counter 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cal_cnt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cal_cnt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CAL_CNT1_SPEC;
impl crate::RegisterSpec for CLK_CAL_CNT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cal_cnt1::R`](R) reader structure"]
impl crate::Readable for CLK_CAL_CNT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_cal_cnt1::W`](W) writer structure"]
impl crate::Writable for CLK_CAL_CNT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CAL_CNT1 to value 0x8000_0000"]
impl crate::Resettable for CLK_CAL_CNT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}

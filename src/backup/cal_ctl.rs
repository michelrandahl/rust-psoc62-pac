#[doc = "Register `CAL_CTL` reader"]
pub type R = crate::R<CAL_CTL_SPEC>;
#[doc = "Register `CAL_CTL` writer"]
pub type W = crate::W<CAL_CTL_SPEC>;
#[doc = "Field `CALIB_VAL` reader - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
pub type CALIB_VAL_R = crate::FieldReader;
#[doc = "Field `CALIB_VAL` writer - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
pub type CALIB_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CALIB_SIGN` reader - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
pub type CALIB_SIGN_R = crate::BitReader;
#[doc = "Field `CALIB_SIGN` writer - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
pub type CALIB_SIGN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_OUT` reader - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
pub type CAL_OUT_R = crate::BitReader;
#[doc = "Field `CAL_OUT` writer - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
pub type CAL_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    pub fn calib_val(&self) -> CALIB_VAL_R {
        CALIB_VAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    pub fn calib_sign(&self) -> CALIB_SIGN_R {
        CALIB_SIGN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    pub fn cal_out(&self) -> CAL_OUT_R {
        CAL_OUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    #[must_use]
    pub fn calib_val(&mut self) -> CALIB_VAL_W<CAL_CTL_SPEC> {
        CALIB_VAL_W::new(self, 0)
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    #[must_use]
    pub fn calib_sign(&mut self) -> CALIB_SIGN_W<CAL_CTL_SPEC> {
        CALIB_SIGN_W::new(self, 6)
    }
    #[doc = "Bit 31 - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    #[must_use]
    pub fn cal_out(&mut self) -> CAL_OUT_W<CAL_CTL_SPEC> {
        CAL_OUT_W::new(self, 31)
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
#[doc = "Oscillator calibration for absolute frequency\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_CTL_SPEC;
impl crate::RegisterSpec for CAL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_ctl::R`](R) reader structure"]
impl crate::Readable for CAL_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal_ctl::W`](W) writer structure"]
impl crate::Writable for CAL_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL to value 0"]
impl crate::Resettable for CAL_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

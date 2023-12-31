#[doc = "Register `OUT_SET` reader"]
pub type R = crate::R<OUT_SET_SPEC>;
#[doc = "Register `OUT_SET` writer"]
pub type W = crate::W<OUT_SET_SPEC>;
#[doc = "Field `OUT0` reader - IO set output for pin 0: '0': Output state not affected. '1': Output state set to '1'."]
pub type OUT0_R = crate::BitReader;
#[doc = "Field `OUT0` writer - IO set output for pin 0: '0': Output state not affected. '1': Output state set to '1'."]
pub type OUT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT1` reader - IO set output for pin 1"]
pub type OUT1_R = crate::BitReader;
#[doc = "Field `OUT1` writer - IO set output for pin 1"]
pub type OUT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT2` reader - IO set output for pin 2"]
pub type OUT2_R = crate::BitReader;
#[doc = "Field `OUT2` writer - IO set output for pin 2"]
pub type OUT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT3` reader - IO set output for pin 3"]
pub type OUT3_R = crate::BitReader;
#[doc = "Field `OUT3` writer - IO set output for pin 3"]
pub type OUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT4` reader - IO set output for pin 4"]
pub type OUT4_R = crate::BitReader;
#[doc = "Field `OUT4` writer - IO set output for pin 4"]
pub type OUT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT5` reader - IO set output for pin 5"]
pub type OUT5_R = crate::BitReader;
#[doc = "Field `OUT5` writer - IO set output for pin 5"]
pub type OUT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT6` reader - IO set output for pin 6"]
pub type OUT6_R = crate::BitReader;
#[doc = "Field `OUT6` writer - IO set output for pin 6"]
pub type OUT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT7` reader - IO set output for pin 7"]
pub type OUT7_R = crate::BitReader;
#[doc = "Field `OUT7` writer - IO set output for pin 7"]
pub type OUT7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO set output for pin 0: '0': Output state not affected. '1': Output state set to '1'."]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO set output for pin 1"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO set output for pin 2"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO set output for pin 3"]
    #[inline(always)]
    pub fn out3(&self) -> OUT3_R {
        OUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO set output for pin 4"]
    #[inline(always)]
    pub fn out4(&self) -> OUT4_R {
        OUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO set output for pin 5"]
    #[inline(always)]
    pub fn out5(&self) -> OUT5_R {
        OUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO set output for pin 6"]
    #[inline(always)]
    pub fn out6(&self) -> OUT6_R {
        OUT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO set output for pin 7"]
    #[inline(always)]
    pub fn out7(&self) -> OUT7_R {
        OUT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO set output for pin 0: '0': Output state not affected. '1': Output state set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> OUT0_W<OUT_SET_SPEC> {
        OUT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - IO set output for pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> OUT1_W<OUT_SET_SPEC> {
        OUT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - IO set output for pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> OUT2_W<OUT_SET_SPEC> {
        OUT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - IO set output for pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn out3(&mut self) -> OUT3_W<OUT_SET_SPEC> {
        OUT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - IO set output for pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn out4(&mut self) -> OUT4_W<OUT_SET_SPEC> {
        OUT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - IO set output for pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn out5(&mut self) -> OUT5_W<OUT_SET_SPEC> {
        OUT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - IO set output for pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn out6(&mut self) -> OUT6_W<OUT_SET_SPEC> {
        OUT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - IO set output for pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn out7(&mut self) -> OUT7_W<OUT_SET_SPEC> {
        OUT7_W::new(self, 7)
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
#[doc = "Port output data set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_SET_SPEC;
impl crate::RegisterSpec for OUT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_set::R`](R) reader structure"]
impl crate::Readable for OUT_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_set::W`](W) writer structure"]
impl crate::Writable for OUT_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_SET to value 0"]
impl crate::Resettable for OUT_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

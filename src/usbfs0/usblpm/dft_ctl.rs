#[doc = "Register `DFT_CTL` reader"]
pub type R = crate::R<DFT_CTL_SPEC>;
#[doc = "Register `DFT_CTL` writer"]
pub type W = crate::W<DFT_CTL_SPEC>;
#[doc = "Field `DDFT_OUT_SEL` reader - DDFT output select signal"]
pub type DDFT_OUT_SEL_R = crate::FieldReader<DDFT_OUT_SEL_A>;
#[doc = "DDFT output select signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DDFT_OUT_SEL_A {
    #[doc = "0: Nothing connected, output 0"]
    OFF = 0,
    #[doc = "1: Single Ended output of DP"]
    DP_SE = 1,
    #[doc = "2: Single Ended output of DM"]
    DM_SE = 2,
    #[doc = "3: Output Enable"]
    TXOE = 3,
    #[doc = "4: Differential Receiver output"]
    RCV_DF = 4,
    #[doc = "5: GPIO output of DP"]
    GPIO_DP_OUT = 5,
    #[doc = "6: GPIO output of DM"]
    GPIO_DM_OUT = 6,
}
impl From<DDFT_OUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DDFT_OUT_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DDFT_OUT_SEL_A {
    type Ux = u8;
}
impl DDFT_OUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DDFT_OUT_SEL_A> {
        match self.bits {
            0 => Some(DDFT_OUT_SEL_A::OFF),
            1 => Some(DDFT_OUT_SEL_A::DP_SE),
            2 => Some(DDFT_OUT_SEL_A::DM_SE),
            3 => Some(DDFT_OUT_SEL_A::TXOE),
            4 => Some(DDFT_OUT_SEL_A::RCV_DF),
            5 => Some(DDFT_OUT_SEL_A::GPIO_DP_OUT),
            6 => Some(DDFT_OUT_SEL_A::GPIO_DM_OUT),
            _ => None,
        }
    }
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DDFT_OUT_SEL_A::OFF
    }
    #[doc = "Single Ended output of DP"]
    #[inline(always)]
    pub fn is_dp_se(&self) -> bool {
        *self == DDFT_OUT_SEL_A::DP_SE
    }
    #[doc = "Single Ended output of DM"]
    #[inline(always)]
    pub fn is_dm_se(&self) -> bool {
        *self == DDFT_OUT_SEL_A::DM_SE
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub fn is_txoe(&self) -> bool {
        *self == DDFT_OUT_SEL_A::TXOE
    }
    #[doc = "Differential Receiver output"]
    #[inline(always)]
    pub fn is_rcv_df(&self) -> bool {
        *self == DDFT_OUT_SEL_A::RCV_DF
    }
    #[doc = "GPIO output of DP"]
    #[inline(always)]
    pub fn is_gpio_dp_out(&self) -> bool {
        *self == DDFT_OUT_SEL_A::GPIO_DP_OUT
    }
    #[doc = "GPIO output of DM"]
    #[inline(always)]
    pub fn is_gpio_dm_out(&self) -> bool {
        *self == DDFT_OUT_SEL_A::GPIO_DM_OUT
    }
}
#[doc = "Field `DDFT_OUT_SEL` writer - DDFT output select signal"]
pub type DDFT_OUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, DDFT_OUT_SEL_A>;
impl<'a, REG> DDFT_OUT_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_OUT_SEL_A::OFF)
    }
    #[doc = "Single Ended output of DP"]
    #[inline(always)]
    pub fn dp_se(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_OUT_SEL_A::DP_SE)
    }
    #[doc = "Single Ended output of DM"]
    #[inline(always)]
    pub fn dm_se(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_OUT_SEL_A::DM_SE)
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub fn txoe(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_OUT_SEL_A::TXOE)
    }
    #[doc = "Differential Receiver output"]
    #[inline(always)]
    pub fn rcv_df(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_OUT_SEL_A::RCV_DF)
    }
    #[doc = "GPIO output of DP"]
    #[inline(always)]
    pub fn gpio_dp_out(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_OUT_SEL_A::GPIO_DP_OUT)
    }
    #[doc = "GPIO output of DM"]
    #[inline(always)]
    pub fn gpio_dm_out(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_OUT_SEL_A::GPIO_DM_OUT)
    }
}
#[doc = "Field `DDFT_IN_SEL` reader - DDFT input select signal"]
pub type DDFT_IN_SEL_R = crate::FieldReader<DDFT_IN_SEL_A>;
#[doc = "DDFT input select signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DDFT_IN_SEL_A {
    #[doc = "0: Nothing connected, output 0"]
    OFF = 0,
    #[doc = "1: GPIO input of DP"]
    GPIO_DP_IN = 1,
    #[doc = "2: GPIO input of DM"]
    GPIO_DM_IN = 2,
}
impl From<DDFT_IN_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DDFT_IN_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DDFT_IN_SEL_A {
    type Ux = u8;
}
impl DDFT_IN_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DDFT_IN_SEL_A> {
        match self.bits {
            0 => Some(DDFT_IN_SEL_A::OFF),
            1 => Some(DDFT_IN_SEL_A::GPIO_DP_IN),
            2 => Some(DDFT_IN_SEL_A::GPIO_DM_IN),
            _ => None,
        }
    }
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DDFT_IN_SEL_A::OFF
    }
    #[doc = "GPIO input of DP"]
    #[inline(always)]
    pub fn is_gpio_dp_in(&self) -> bool {
        *self == DDFT_IN_SEL_A::GPIO_DP_IN
    }
    #[doc = "GPIO input of DM"]
    #[inline(always)]
    pub fn is_gpio_dm_in(&self) -> bool {
        *self == DDFT_IN_SEL_A::GPIO_DM_IN
    }
}
#[doc = "Field `DDFT_IN_SEL` writer - DDFT input select signal"]
pub type DDFT_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DDFT_IN_SEL_A>;
impl<'a, REG> DDFT_IN_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_IN_SEL_A::OFF)
    }
    #[doc = "GPIO input of DP"]
    #[inline(always)]
    pub fn gpio_dp_in(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_IN_SEL_A::GPIO_DP_IN)
    }
    #[doc = "GPIO input of DM"]
    #[inline(always)]
    pub fn gpio_dm_in(self) -> &'a mut crate::W<REG> {
        self.variant(DDFT_IN_SEL_A::GPIO_DM_IN)
    }
}
impl R {
    #[doc = "Bits 0:2 - DDFT output select signal"]
    #[inline(always)]
    pub fn ddft_out_sel(&self) -> DDFT_OUT_SEL_R {
        DDFT_OUT_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - DDFT input select signal"]
    #[inline(always)]
    pub fn ddft_in_sel(&self) -> DDFT_IN_SEL_R {
        DDFT_IN_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DDFT output select signal"]
    #[inline(always)]
    #[must_use]
    pub fn ddft_out_sel(&mut self) -> DDFT_OUT_SEL_W<DFT_CTL_SPEC> {
        DDFT_OUT_SEL_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - DDFT input select signal"]
    #[inline(always)]
    #[must_use]
    pub fn ddft_in_sel(&mut self) -> DDFT_IN_SEL_W<DFT_CTL_SPEC> {
        DDFT_IN_SEL_W::new(self, 3)
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
#[doc = "DFT control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dft_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dft_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFT_CTL_SPEC;
impl crate::RegisterSpec for DFT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_ctl::R`](R) reader structure"]
impl crate::Readable for DFT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dft_ctl::W`](W) writer structure"]
impl crate::Writable for DFT_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFT_CTL to value 0"]
impl crate::Resettable for DFT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

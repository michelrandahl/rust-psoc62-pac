#[doc = "Register `CLK_DSI_SELECT[%s]` reader"]
pub type R = crate::R<CLK_DSI_SELECT_SPEC>;
#[doc = "Register `CLK_DSI_SELECT[%s]` writer"]
pub type W = crate::W<CLK_DSI_SELECT_SPEC>;
#[doc = "Field `DSI_MUX` reader - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
pub type DSI_MUX_R = crate::FieldReader<DSI_MUX_A>;
#[doc = "Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSI_MUX_A {
    #[doc = "0: DSI0 - dsi_out\\[0\\]"]
    DSI_OUT0 = 0,
    #[doc = "1: DSI1 - dsi_out\\[1\\]"]
    DSI_OUT1 = 1,
    #[doc = "2: DSI2 - dsi_out\\[2\\]"]
    DSI_OUT2 = 2,
    #[doc = "3: DSI3 - dsi_out\\[3\\]"]
    DSI_OUT3 = 3,
    #[doc = "4: DSI4 - dsi_out\\[4\\]"]
    DSI_OUT4 = 4,
    #[doc = "5: DSI5 - dsi_out\\[5\\]"]
    DSI_OUT5 = 5,
    #[doc = "6: DSI6 - dsi_out\\[6\\]"]
    DSI_OUT6 = 6,
    #[doc = "7: DSI7 - dsi_out\\[7\\]"]
    DSI_OUT7 = 7,
    #[doc = "8: DSI8 - dsi_out\\[8\\]"]
    DSI_OUT8 = 8,
    #[doc = "9: DSI9 - dsi_out\\[9\\]"]
    DSI_OUT9 = 9,
    #[doc = "10: DSI10 - dsi_out\\[10\\]"]
    DSI_OUT10 = 10,
    #[doc = "11: DSI11 - dsi_out\\[11\\]"]
    DSI_OUT11 = 11,
    #[doc = "12: DSI12 - dsi_out\\[12\\]"]
    DSI_OUT12 = 12,
    #[doc = "13: DSI13 - dsi_out\\[13\\]"]
    DSI_OUT13 = 13,
    #[doc = "14: DSI14 - dsi_out\\[14\\]"]
    DSI_OUT14 = 14,
    #[doc = "15: DSI15 - dsi_out\\[15\\]"]
    DSI_OUT15 = 15,
    #[doc = "16: ILO - Internal Low-speed Oscillator"]
    ILO = 16,
    #[doc = "17: WCO - Watch-Crystal Oscillator"]
    WCO = 17,
    #[doc = "18: ALTLF - Alternate Low-Frequency Clock"]
    ALTLF = 18,
    #[doc = "19: PILO - Precision Internal Low-speed Oscillator"]
    PILO = 19,
}
impl From<DSI_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: DSI_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSI_MUX_A {
    type Ux = u8;
}
impl DSI_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DSI_MUX_A> {
        match self.bits {
            0 => Some(DSI_MUX_A::DSI_OUT0),
            1 => Some(DSI_MUX_A::DSI_OUT1),
            2 => Some(DSI_MUX_A::DSI_OUT2),
            3 => Some(DSI_MUX_A::DSI_OUT3),
            4 => Some(DSI_MUX_A::DSI_OUT4),
            5 => Some(DSI_MUX_A::DSI_OUT5),
            6 => Some(DSI_MUX_A::DSI_OUT6),
            7 => Some(DSI_MUX_A::DSI_OUT7),
            8 => Some(DSI_MUX_A::DSI_OUT8),
            9 => Some(DSI_MUX_A::DSI_OUT9),
            10 => Some(DSI_MUX_A::DSI_OUT10),
            11 => Some(DSI_MUX_A::DSI_OUT11),
            12 => Some(DSI_MUX_A::DSI_OUT12),
            13 => Some(DSI_MUX_A::DSI_OUT13),
            14 => Some(DSI_MUX_A::DSI_OUT14),
            15 => Some(DSI_MUX_A::DSI_OUT15),
            16 => Some(DSI_MUX_A::ILO),
            17 => Some(DSI_MUX_A::WCO),
            18 => Some(DSI_MUX_A::ALTLF),
            19 => Some(DSI_MUX_A::PILO),
            _ => None,
        }
    }
    #[doc = "DSI0 - dsi_out\\[0\\]"]
    #[inline(always)]
    pub fn is_dsi_out0(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT0
    }
    #[doc = "DSI1 - dsi_out\\[1\\]"]
    #[inline(always)]
    pub fn is_dsi_out1(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT1
    }
    #[doc = "DSI2 - dsi_out\\[2\\]"]
    #[inline(always)]
    pub fn is_dsi_out2(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT2
    }
    #[doc = "DSI3 - dsi_out\\[3\\]"]
    #[inline(always)]
    pub fn is_dsi_out3(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT3
    }
    #[doc = "DSI4 - dsi_out\\[4\\]"]
    #[inline(always)]
    pub fn is_dsi_out4(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT4
    }
    #[doc = "DSI5 - dsi_out\\[5\\]"]
    #[inline(always)]
    pub fn is_dsi_out5(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT5
    }
    #[doc = "DSI6 - dsi_out\\[6\\]"]
    #[inline(always)]
    pub fn is_dsi_out6(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT6
    }
    #[doc = "DSI7 - dsi_out\\[7\\]"]
    #[inline(always)]
    pub fn is_dsi_out7(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT7
    }
    #[doc = "DSI8 - dsi_out\\[8\\]"]
    #[inline(always)]
    pub fn is_dsi_out8(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT8
    }
    #[doc = "DSI9 - dsi_out\\[9\\]"]
    #[inline(always)]
    pub fn is_dsi_out9(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT9
    }
    #[doc = "DSI10 - dsi_out\\[10\\]"]
    #[inline(always)]
    pub fn is_dsi_out10(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT10
    }
    #[doc = "DSI11 - dsi_out\\[11\\]"]
    #[inline(always)]
    pub fn is_dsi_out11(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT11
    }
    #[doc = "DSI12 - dsi_out\\[12\\]"]
    #[inline(always)]
    pub fn is_dsi_out12(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT12
    }
    #[doc = "DSI13 - dsi_out\\[13\\]"]
    #[inline(always)]
    pub fn is_dsi_out13(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT13
    }
    #[doc = "DSI14 - dsi_out\\[14\\]"]
    #[inline(always)]
    pub fn is_dsi_out14(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT14
    }
    #[doc = "DSI15 - dsi_out\\[15\\]"]
    #[inline(always)]
    pub fn is_dsi_out15(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT15
    }
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == DSI_MUX_A::ILO
    }
    #[doc = "WCO - Watch-Crystal Oscillator"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == DSI_MUX_A::WCO
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == DSI_MUX_A::ALTLF
    }
    #[doc = "PILO - Precision Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == DSI_MUX_A::PILO
    }
}
#[doc = "Field `DSI_MUX` writer - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
pub type DSI_MUX_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DSI_MUX_A>;
impl<'a, REG> DSI_MUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DSI0 - dsi_out\\[0\\]"]
    #[inline(always)]
    pub fn dsi_out0(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT0)
    }
    #[doc = "DSI1 - dsi_out\\[1\\]"]
    #[inline(always)]
    pub fn dsi_out1(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT1)
    }
    #[doc = "DSI2 - dsi_out\\[2\\]"]
    #[inline(always)]
    pub fn dsi_out2(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT2)
    }
    #[doc = "DSI3 - dsi_out\\[3\\]"]
    #[inline(always)]
    pub fn dsi_out3(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT3)
    }
    #[doc = "DSI4 - dsi_out\\[4\\]"]
    #[inline(always)]
    pub fn dsi_out4(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT4)
    }
    #[doc = "DSI5 - dsi_out\\[5\\]"]
    #[inline(always)]
    pub fn dsi_out5(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT5)
    }
    #[doc = "DSI6 - dsi_out\\[6\\]"]
    #[inline(always)]
    pub fn dsi_out6(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT6)
    }
    #[doc = "DSI7 - dsi_out\\[7\\]"]
    #[inline(always)]
    pub fn dsi_out7(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT7)
    }
    #[doc = "DSI8 - dsi_out\\[8\\]"]
    #[inline(always)]
    pub fn dsi_out8(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT8)
    }
    #[doc = "DSI9 - dsi_out\\[9\\]"]
    #[inline(always)]
    pub fn dsi_out9(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT9)
    }
    #[doc = "DSI10 - dsi_out\\[10\\]"]
    #[inline(always)]
    pub fn dsi_out10(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT10)
    }
    #[doc = "DSI11 - dsi_out\\[11\\]"]
    #[inline(always)]
    pub fn dsi_out11(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT11)
    }
    #[doc = "DSI12 - dsi_out\\[12\\]"]
    #[inline(always)]
    pub fn dsi_out12(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT12)
    }
    #[doc = "DSI13 - dsi_out\\[13\\]"]
    #[inline(always)]
    pub fn dsi_out13(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT13)
    }
    #[doc = "DSI14 - dsi_out\\[14\\]"]
    #[inline(always)]
    pub fn dsi_out14(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT14)
    }
    #[doc = "DSI15 - dsi_out\\[15\\]"]
    #[inline(always)]
    pub fn dsi_out15(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::DSI_OUT15)
    }
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::ILO)
    }
    #[doc = "WCO - Watch-Crystal Oscillator"]
    #[inline(always)]
    pub fn wco(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::WCO)
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::ALTLF)
    }
    #[doc = "PILO - Precision Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut crate::W<REG> {
        self.variant(DSI_MUX_A::PILO)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
    #[inline(always)]
    pub fn dsi_mux(&self) -> DSI_MUX_R {
        DSI_MUX_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_mux(&mut self) -> DSI_MUX_W<CLK_DSI_SELECT_SPEC> {
        DSI_MUX_W::new(self, 0)
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
#[doc = "Clock DSI Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dsi_select::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dsi_select::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_DSI_SELECT_SPEC;
impl crate::RegisterSpec for CLK_DSI_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_dsi_select::R`](R) reader structure"]
impl crate::Readable for CLK_DSI_SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_dsi_select::W`](W) writer structure"]
impl crate::Writable for CLK_DSI_SELECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_DSI_SELECT[%s]
to value 0"]
impl crate::Resettable for CLK_DSI_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

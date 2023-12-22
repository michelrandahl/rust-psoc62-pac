#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `WCO_EN` reader - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
pub type WCO_EN_R = crate::BitReader;
#[doc = "Field `WCO_EN` writer - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
pub type WCO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SEL` reader - Clock select for BAK clock"]
pub type CLK_SEL_R = crate::FieldReader<CLK_SEL_A>;
#[doc = "Clock select for BAK clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SEL_A {
    #[doc = "0: Watch-crystal oscillator input."]
    WCO = 0,
    #[doc = "1: This allows to use the LFCLK selection as an alternate backup domain clock. Note that LFCLK is not available in all power modes, and clock glitches can propagate into the backup logic when the clock is stopped. For this reason, if the WCO is intended as the clock source then choose it directly instead of routing through LFCLK."]
    ALTBAK = 1,
}
impl From<CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLK_SEL_A {
    type Ux = u8;
}
impl CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLK_SEL_A> {
        match self.bits {
            0 => Some(CLK_SEL_A::WCO),
            1 => Some(CLK_SEL_A::ALTBAK),
            _ => None,
        }
    }
    #[doc = "Watch-crystal oscillator input."]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == CLK_SEL_A::WCO
    }
    #[doc = "This allows to use the LFCLK selection as an alternate backup domain clock. Note that LFCLK is not available in all power modes, and clock glitches can propagate into the backup logic when the clock is stopped. For this reason, if the WCO is intended as the clock source then choose it directly instead of routing through LFCLK."]
    #[inline(always)]
    pub fn is_altbak(&self) -> bool {
        *self == CLK_SEL_A::ALTBAK
    }
}
#[doc = "Field `CLK_SEL` writer - Clock select for BAK clock"]
pub type CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLK_SEL_A>;
impl<'a, REG> CLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Watch-crystal oscillator input."]
    #[inline(always)]
    pub fn wco(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SEL_A::WCO)
    }
    #[doc = "This allows to use the LFCLK selection as an alternate backup domain clock. Note that LFCLK is not available in all power modes, and clock glitches can propagate into the backup logic when the clock is stopped. For this reason, if the WCO is intended as the clock source then choose it directly instead of routing through LFCLK."]
    #[inline(always)]
    pub fn altbak(self) -> &'a mut crate::W<REG> {
        self.variant(CLK_SEL_A::ALTBAK)
    }
}
#[doc = "Field `PRESCALER` reader - N/A"]
pub type PRESCALER_R = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - N/A"]
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WCO_BYPASS` reader - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
pub type WCO_BYPASS_R = crate::BitReader;
#[doc = "Field `WCO_BYPASS` writer - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
pub type WCO_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAK_CTL` reader - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
pub type VDDBAK_CTL_R = crate::FieldReader;
#[doc = "Field `VDDBAK_CTL` writer - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
pub type VDDBAK_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VBACKUP_MEAS` reader - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
pub type VBACKUP_MEAS_R = crate::BitReader;
#[doc = "Field `VBACKUP_MEAS` writer - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
pub type VBACKUP_MEAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CHARGE_KEY` reader - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
pub type EN_CHARGE_KEY_R = crate::FieldReader;
#[doc = "Field `EN_CHARGE_KEY` writer - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
pub type EN_CHARGE_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 3 - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
    #[inline(always)]
    pub fn wco_en(&self) -> WCO_EN_R {
        WCO_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock select for BAK clock"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
    #[inline(always)]
    pub fn wco_bypass(&self) -> WCO_BYPASS_R {
        WCO_BYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
    #[inline(always)]
    pub fn vddbak_ctl(&self) -> VDDBAK_CTL_R {
        VDDBAK_CTL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
    #[inline(always)]
    pub fn vbackup_meas(&self) -> VBACKUP_MEAS_R {
        VBACKUP_MEAS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
    #[inline(always)]
    pub fn en_charge_key(&self) -> EN_CHARGE_KEY_R {
        EN_CHARGE_KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
    #[inline(always)]
    #[must_use]
    pub fn wco_en(&mut self) -> WCO_EN_W<CTL_SPEC> {
        WCO_EN_W::new(self, 3)
    }
    #[doc = "Bits 8:9 - Clock select for BAK clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<CTL_SPEC> {
        CLK_SEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<CTL_SPEC> {
        PRESCALER_W::new(self, 12)
    }
    #[doc = "Bit 16 - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
    #[inline(always)]
    #[must_use]
    pub fn wco_bypass(&mut self) -> WCO_BYPASS_W<CTL_SPEC> {
        WCO_BYPASS_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
    #[inline(always)]
    #[must_use]
    pub fn vddbak_ctl(&mut self) -> VDDBAK_CTL_W<CTL_SPEC> {
        VDDBAK_CTL_W::new(self, 17)
    }
    #[doc = "Bit 19 - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
    #[inline(always)]
    #[must_use]
    pub fn vbackup_meas(&mut self) -> VBACKUP_MEAS_W<CTL_SPEC> {
        VBACKUP_MEAS_W::new(self, 19)
    }
    #[doc = "Bits 24:31 - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
    #[inline(always)]
    #[must_use]
    pub fn en_charge_key(&mut self) -> EN_CHARGE_KEY_W<CTL_SPEC> {
        EN_CHARGE_KEY_W::new(self, 24)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

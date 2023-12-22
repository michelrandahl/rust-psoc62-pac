#[doc = "Register `PWR_BUCK_CTL` reader"]
pub type R = crate::R<PWR_BUCK_CTL_SPEC>;
#[doc = "Register `PWR_BUCK_CTL` writer"]
pub type W = crate::W<PWR_BUCK_CTL_SPEC>;
#[doc = "Field `BUCK_OUT1_SEL` reader - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
pub type BUCK_OUT1_SEL_R = crate::FieldReader;
#[doc = "Field `BUCK_OUT1_SEL` writer - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
pub type BUCK_OUT1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUCK_EN` reader - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type BUCK_EN_R = crate::BitReader;
#[doc = "Field `BUCK_EN` writer - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type BUCK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUCK_OUT1_EN` reader - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
pub type BUCK_OUT1_EN_R = crate::BitReader;
#[doc = "Field `BUCK_OUT1_EN` writer - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
pub type BUCK_OUT1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    pub fn buck_out1_sel(&self) -> BUCK_OUT1_SEL_R {
        BUCK_OUT1_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 30 - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn buck_en(&self) -> BUCK_EN_R {
        BUCK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
    #[inline(always)]
    pub fn buck_out1_en(&self) -> BUCK_OUT1_EN_R {
        BUCK_OUT1_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    #[must_use]
    pub fn buck_out1_sel(&mut self) -> BUCK_OUT1_SEL_W<PWR_BUCK_CTL_SPEC> {
        BUCK_OUT1_SEL_W::new(self, 0)
    }
    #[doc = "Bit 30 - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn buck_en(&mut self) -> BUCK_EN_W<PWR_BUCK_CTL_SPEC> {
        BUCK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
    #[inline(always)]
    #[must_use]
    pub fn buck_out1_en(&mut self) -> BUCK_OUT1_EN_W<PWR_BUCK_CTL_SPEC> {
        BUCK_OUT1_EN_W::new(self, 31)
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
#[doc = "Buck Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_buck_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_buck_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_BUCK_CTL_SPEC;
impl crate::RegisterSpec for PWR_BUCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_buck_ctl::R`](R) reader structure"]
impl crate::Readable for PWR_BUCK_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_buck_ctl::W`](W) writer structure"]
impl crate::Writable for PWR_BUCK_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_BUCK_CTL to value 0x05"]
impl crate::Resettable for PWR_BUCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}

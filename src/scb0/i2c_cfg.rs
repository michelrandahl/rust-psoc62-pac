#[doc = "Register `I2C_CFG` reader"]
pub type R = crate::R<I2C_CFG_SPEC>;
#[doc = "Register `I2C_CFG` writer"]
pub type W = crate::W<I2C_CFG_SPEC>;
#[doc = "Field `SDA_IN_FILT_TRIM` reader - Trim bits for 'i2c_sda_in' 50 ns filter. SDA_IN_FILT_TRIM\\[1\\]
is used to enable I2CS_EC or SPIS_EC access to internal EZ memory. 1: enable clk_scb 0: disable clk_scb Before going to deepsleep this field should be set to 0. It should be re-enabled once the device is awoken and clk_hf\\[0\\]
is at the desired frequency."]
pub type SDA_IN_FILT_TRIM_R = crate::FieldReader;
#[doc = "Field `SDA_IN_FILT_TRIM` writer - Trim bits for 'i2c_sda_in' 50 ns filter. SDA_IN_FILT_TRIM\\[1\\]
is used to enable I2CS_EC or SPIS_EC access to internal EZ memory. 1: enable clk_scb 0: disable clk_scb Before going to deepsleep this field should be set to 0. It should be re-enabled once the device is awoken and clk_hf\\[0\\]
is at the desired frequency."]
pub type SDA_IN_FILT_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDA_IN_FILT_SEL` reader - N/A"]
pub type SDA_IN_FILT_SEL_R = crate::BitReader;
#[doc = "Field `SDA_IN_FILT_SEL` writer - N/A"]
pub type SDA_IN_FILT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_IN_FILT_TRIM` reader - Trim bits for 'i2c_scl_in' 50 ns filter. Not to be modified by the user"]
pub type SCL_IN_FILT_TRIM_R = crate::FieldReader;
#[doc = "Field `SCL_IN_FILT_TRIM` writer - Trim bits for 'i2c_scl_in' 50 ns filter. Not to be modified by the user"]
pub type SCL_IN_FILT_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCL_IN_FILT_SEL` reader - N/A"]
pub type SCL_IN_FILT_SEL_R = crate::BitReader;
#[doc = "Field `SCL_IN_FILT_SEL` writer - N/A"]
pub type SCL_IN_FILT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDA_OUT_FILT0_TRIM` reader - Trim bits for 'i2c_sda_out' 50 ns filter 0. Not to be modified by the user"]
pub type SDA_OUT_FILT0_TRIM_R = crate::FieldReader;
#[doc = "Field `SDA_OUT_FILT0_TRIM` writer - Trim bits for 'i2c_sda_out' 50 ns filter 0. Not to be modified by the user"]
pub type SDA_OUT_FILT0_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDA_OUT_FILT1_TRIM` reader - Trim bits for 'i2c_sda_out' 50 ns filter 1. Not to be modified by the user"]
pub type SDA_OUT_FILT1_TRIM_R = crate::FieldReader;
#[doc = "Field `SDA_OUT_FILT1_TRIM` writer - Trim bits for 'i2c_sda_out' 50 ns filter 1. Not to be modified by the user"]
pub type SDA_OUT_FILT1_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDA_OUT_FILT2_TRIM` reader - Trim bits for 'i2c_sda_out' 50 ns filter 2. Not to be modified by the user"]
pub type SDA_OUT_FILT2_TRIM_R = crate::FieldReader;
#[doc = "Field `SDA_OUT_FILT2_TRIM` writer - Trim bits for 'i2c_sda_out' 50 ns filter 2. Not to be modified by the user"]
pub type SDA_OUT_FILT2_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDA_OUT_FILT_SEL` reader - N/A"]
pub type SDA_OUT_FILT_SEL_R = crate::FieldReader;
#[doc = "Field `SDA_OUT_FILT_SEL` writer - N/A"]
pub type SDA_OUT_FILT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Trim bits for 'i2c_sda_in' 50 ns filter. SDA_IN_FILT_TRIM\\[1\\]
is used to enable I2CS_EC or SPIS_EC access to internal EZ memory. 1: enable clk_scb 0: disable clk_scb Before going to deepsleep this field should be set to 0. It should be re-enabled once the device is awoken and clk_hf\\[0\\]
is at the desired frequency."]
    #[inline(always)]
    pub fn sda_in_filt_trim(&self) -> SDA_IN_FILT_TRIM_R {
        SDA_IN_FILT_TRIM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn sda_in_filt_sel(&self) -> SDA_IN_FILT_SEL_R {
        SDA_IN_FILT_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Trim bits for 'i2c_scl_in' 50 ns filter. Not to be modified by the user"]
    #[inline(always)]
    pub fn scl_in_filt_trim(&self) -> SCL_IN_FILT_TRIM_R {
        SCL_IN_FILT_TRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn scl_in_filt_sel(&self) -> SCL_IN_FILT_SEL_R {
        SCL_IN_FILT_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Trim bits for 'i2c_sda_out' 50 ns filter 0. Not to be modified by the user"]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&self) -> SDA_OUT_FILT0_TRIM_R {
        SDA_OUT_FILT0_TRIM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim bits for 'i2c_sda_out' 50 ns filter 1. Not to be modified by the user"]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&self) -> SDA_OUT_FILT1_TRIM_R {
        SDA_OUT_FILT1_TRIM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Trim bits for 'i2c_sda_out' 50 ns filter 2. Not to be modified by the user"]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&self) -> SDA_OUT_FILT2_TRIM_R {
        SDA_OUT_FILT2_TRIM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - N/A"]
    #[inline(always)]
    pub fn sda_out_filt_sel(&self) -> SDA_OUT_FILT_SEL_R {
        SDA_OUT_FILT_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trim bits for 'i2c_sda_in' 50 ns filter. SDA_IN_FILT_TRIM\\[1\\]
is used to enable I2CS_EC or SPIS_EC access to internal EZ memory. 1: enable clk_scb 0: disable clk_scb Before going to deepsleep this field should be set to 0. It should be re-enabled once the device is awoken and clk_hf\\[0\\]
is at the desired frequency."]
    #[inline(always)]
    #[must_use]
    pub fn sda_in_filt_trim(&mut self) -> SDA_IN_FILT_TRIM_W<I2C_CFG_SPEC> {
        SDA_IN_FILT_TRIM_W::new(self, 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sda_in_filt_sel(&mut self) -> SDA_IN_FILT_SEL_W<I2C_CFG_SPEC> {
        SDA_IN_FILT_SEL_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Trim bits for 'i2c_scl_in' 50 ns filter. Not to be modified by the user"]
    #[inline(always)]
    #[must_use]
    pub fn scl_in_filt_trim(&mut self) -> SCL_IN_FILT_TRIM_W<I2C_CFG_SPEC> {
        SCL_IN_FILT_TRIM_W::new(self, 8)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn scl_in_filt_sel(&mut self) -> SCL_IN_FILT_SEL_W<I2C_CFG_SPEC> {
        SCL_IN_FILT_SEL_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Trim bits for 'i2c_sda_out' 50 ns filter 0. Not to be modified by the user"]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt0_trim(&mut self) -> SDA_OUT_FILT0_TRIM_W<I2C_CFG_SPEC> {
        SDA_OUT_FILT0_TRIM_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Trim bits for 'i2c_sda_out' 50 ns filter 1. Not to be modified by the user"]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt1_trim(&mut self) -> SDA_OUT_FILT1_TRIM_W<I2C_CFG_SPEC> {
        SDA_OUT_FILT1_TRIM_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Trim bits for 'i2c_sda_out' 50 ns filter 2. Not to be modified by the user"]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt2_trim(&mut self) -> SDA_OUT_FILT2_TRIM_W<I2C_CFG_SPEC> {
        SDA_OUT_FILT2_TRIM_W::new(self, 20)
    }
    #[doc = "Bits 28:29 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt_sel(&mut self) -> SDA_OUT_FILT_SEL_W<I2C_CFG_SPEC> {
        SDA_OUT_FILT_SEL_W::new(self, 28)
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
#[doc = "I2C configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_CFG_SPEC;
impl crate::RegisterSpec for I2C_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_cfg::R`](R) reader structure"]
impl crate::Readable for I2C_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_cfg::W`](W) writer structure"]
impl crate::Writable for I2C_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_CFG to value 0x002a_1013"]
impl crate::Resettable for I2C_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x002a_1013;
}

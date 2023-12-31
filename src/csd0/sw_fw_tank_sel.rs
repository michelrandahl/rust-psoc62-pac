#[doc = "Register `SW_FW_TANK_SEL` reader"]
pub type R = crate::R<SW_FW_TANK_SEL_SPEC>;
#[doc = "Register `SW_FW_TANK_SEL` writer"]
pub type W = crate::W<SW_FW_TANK_SEL_SPEC>;
#[doc = "Field `SW_F2PT` reader - Set corresponding switch"]
pub type SW_F2PT_R = crate::BitReader;
#[doc = "Field `SW_F2PT` writer - Set corresponding switch"]
pub type SW_F2PT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_F2MA` reader - Select waveform for corresponding switch"]
pub type SW_F2MA_R = crate::FieldReader;
#[doc = "Field `SW_F2MA` writer - Select waveform for corresponding switch"]
pub type SW_F2MA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_F2CA` reader - Select waveform for corresponding switch"]
pub type SW_F2CA_R = crate::FieldReader;
#[doc = "Field `SW_F2CA` writer - Select waveform for corresponding switch"]
pub type SW_F2CA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_F2CB` reader - Select waveform for corresponding switch"]
pub type SW_F2CB_R = crate::FieldReader;
#[doc = "Field `SW_F2CB` writer - Select waveform for corresponding switch"]
pub type SW_F2CB_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_C2CC` reader - Set corresponding switch"]
pub type SW_C2CC_R = crate::BitReader;
#[doc = "Field `SW_C2CC` writer - Set corresponding switch"]
pub type SW_C2CC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_C2CD` reader - Set corresponding switch"]
pub type SW_C2CD_R = crate::BitReader;
#[doc = "Field `SW_C2CD` writer - Set corresponding switch"]
pub type SW_C2CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_C2F2` reader - Set corresponding switch"]
pub type SW_C2F2_R = crate::BitReader;
#[doc = "Field `SW_C2F2` writer - Set corresponding switch"]
pub type SW_C2F2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f2pt(&self) -> SW_F2PT_R {
        SW_F2PT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ma(&self) -> SW_F2MA_R {
        SW_F2MA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ca(&self) -> SW_F2CA_R {
        SW_F2CA_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2cb(&self) -> SW_F2CB_R {
        SW_F2CB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cc(&self) -> SW_C2CC_R {
        SW_C2CC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cd(&self) -> SW_C2CD_R {
        SW_C2CD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2f2(&self) -> SW_C2F2_R {
        SW_C2F2_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f2pt(&mut self) -> SW_F2PT_W<SW_FW_TANK_SEL_SPEC> {
        SW_F2PT_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f2ma(&mut self) -> SW_F2MA_W<SW_FW_TANK_SEL_SPEC> {
        SW_F2MA_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f2ca(&mut self) -> SW_F2CA_W<SW_FW_TANK_SEL_SPEC> {
        SW_F2CA_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f2cb(&mut self) -> SW_F2CB_W<SW_FW_TANK_SEL_SPEC> {
        SW_F2CB_W::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c2cc(&mut self) -> SW_C2CC_W<SW_FW_TANK_SEL_SPEC> {
        SW_C2CC_W::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c2cd(&mut self) -> SW_C2CD_W<SW_FW_TANK_SEL_SPEC> {
        SW_C2CD_W::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c2f2(&mut self) -> SW_C2F2_W<SW_FW_TANK_SEL_SPEC> {
        SW_C2F2_W::new(self, 28)
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
#[doc = "Full Wave Csh_tank Switch Waveform selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_fw_tank_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_fw_tank_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_FW_TANK_SEL_SPEC;
impl crate::RegisterSpec for SW_FW_TANK_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_fw_tank_sel::R`](R) reader structure"]
impl crate::Readable for SW_FW_TANK_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_fw_tank_sel::W`](W) writer structure"]
impl crate::Writable for SW_FW_TANK_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_FW_TANK_SEL to value 0"]
impl crate::Resettable for SW_FW_TANK_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

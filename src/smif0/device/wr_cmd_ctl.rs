#[doc = "Register `WR_CMD_CTL` reader"]
pub type R = crate::R<WR_CMD_CTL_SPEC>;
#[doc = "Register `WR_CMD_CTL` writer"]
pub type W = crate::W<WR_CMD_CTL_SPEC>;
#[doc = "Field `CODE` reader - Command byte code."]
pub type CODE_R = crate::FieldReader;
#[doc = "Field `CODE` writer - Command byte code."]
pub type CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub type WIDTH_R = crate::FieldReader;
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub type WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESENT` reader - Presence of command field: '0': not present '1': present"]
pub type PRESENT_R = crate::BitReader;
#[doc = "Field `PRESENT` writer - Presence of command field: '0': not present '1': present"]
pub type PRESENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command byte code."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command byte code."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<WR_CMD_CTL_SPEC> {
        CODE_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<WR_CMD_CTL_SPEC> {
        WIDTH_W::new(self, 16)
    }
    #[doc = "Bit 31 - Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PRESENT_W<WR_CMD_CTL_SPEC> {
        PRESENT_W::new(self, 31)
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
#[doc = "Write command control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_cmd_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_cmd_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_CMD_CTL_SPEC;
impl crate::RegisterSpec for WR_CMD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_cmd_ctl::R`](R) reader structure"]
impl crate::Readable for WR_CMD_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_cmd_ctl::W`](W) writer structure"]
impl crate::Writable for WR_CMD_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_CMD_CTL to value 0"]
impl crate::Resettable for WR_CMD_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

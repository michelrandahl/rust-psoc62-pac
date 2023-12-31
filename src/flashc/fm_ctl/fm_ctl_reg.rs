#[doc = "Register `FM_CTL_REG` reader"]
pub type R = crate::R<FM_CTL_REG_SPEC>;
#[doc = "Register `FM_CTL_REG` writer"]
pub type W = crate::W<FM_CTL_REG_SPEC>;
#[doc = "Field `FM_MODE` reader - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
pub type FM_MODE_R = crate::FieldReader;
#[doc = "Field `FM_MODE` writer - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
pub type FM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FM_SEQ` reader - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
pub type FM_SEQ_R = crate::FieldReader;
#[doc = "Field `FM_SEQ` writer - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
pub type FM_SEQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAA_MUX_SEL` reader - Direct memory cell access address."]
pub type DAA_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `DAA_MUX_SEL` writer - Direct memory cell access address."]
pub type DAA_MUX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IF_SEL` reader - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type IF_SEL_R = crate::BitReader;
#[doc = "Field `IF_SEL` writer - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type IF_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_EN` reader - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type WR_EN_R = crate::BitReader;
#[doc = "Field `WR_EN` writer - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
    #[inline(always)]
    pub fn fm_mode(&self) -> FM_MODE_R {
        FM_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
    #[inline(always)]
    pub fn fm_seq(&self) -> FM_SEQ_R {
        FM_SEQ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    pub fn daa_mux_sel(&self) -> DAA_MUX_SEL_R {
        DAA_MUX_SEL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub fn if_sel(&self) -> IF_SEL_R {
        IF_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn fm_mode(&mut self) -> FM_MODE_W<FM_CTL_REG_SPEC> {
        FM_MODE_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
    #[inline(always)]
    #[must_use]
    pub fn fm_seq(&mut self) -> FM_SEQ_W<FM_CTL_REG_SPEC> {
        FM_SEQ_W::new(self, 8)
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    #[must_use]
    pub fn daa_mux_sel(&mut self) -> DAA_MUX_SEL_W<FM_CTL_REG_SPEC> {
        DAA_MUX_SEL_W::new(self, 16)
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn if_sel(&mut self) -> IF_SEL_W<FM_CTL_REG_SPEC> {
        IF_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WR_EN_W<FM_CTL_REG_SPEC> {
        WR_EN_W::new(self, 25)
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
#[doc = "Flash macro control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_ctl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fm_ctl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FM_CTL_REG_SPEC;
impl crate::RegisterSpec for FM_CTL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_ctl_reg::R`](R) reader structure"]
impl crate::Readable for FM_CTL_REG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fm_ctl_reg::W`](W) writer structure"]
impl crate::Writable for FM_CTL_REG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FM_CTL_REG to value 0"]
impl crate::Resettable for FM_CTL_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

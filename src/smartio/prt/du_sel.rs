#[doc = "Register `DU_SEL` reader"]
pub type R = crate::R<DU_SEL_SPEC>;
#[doc = "Register `DU_SEL` writer"]
pub type W = crate::W<DU_SEL_SPEC>;
#[doc = "Field `DU_TR0_SEL` reader - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
pub type DU_TR0_SEL_R = crate::FieldReader;
#[doc = "Field `DU_TR0_SEL` writer - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
pub type DU_TR0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DU_TR1_SEL` reader - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DU_TR1_SEL_R = crate::FieldReader;
#[doc = "Field `DU_TR1_SEL` writer - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DU_TR1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DU_TR2_SEL` reader - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DU_TR2_SEL_R = crate::FieldReader;
#[doc = "Field `DU_TR2_SEL` writer - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DU_TR2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DU_DATA0_SEL` reader - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
pub type DU_DATA0_SEL_R = crate::FieldReader;
#[doc = "Field `DU_DATA0_SEL` writer - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
pub type DU_DATA0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DU_DATA1_SEL` reader - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
pub type DU_DATA1_SEL_R = crate::FieldReader;
#[doc = "Field `DU_DATA1_SEL` writer - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
pub type DU_DATA1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_tr0_sel(&self) -> DU_TR0_SEL_R {
        DU_TR0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr1_sel(&self) -> DU_TR1_SEL_R {
        DU_TR1_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr2_sel(&self) -> DU_TR2_SEL_R {
        DU_TR2_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
    #[inline(always)]
    pub fn du_data0_sel(&self) -> DU_DATA0_SEL_R {
        DU_DATA0_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    pub fn du_data1_sel(&self) -> DU_DATA1_SEL_R {
        DU_DATA1_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
    #[inline(always)]
    #[must_use]
    pub fn du_tr0_sel(&mut self) -> DU_TR0_SEL_W<DU_SEL_SPEC> {
        DU_TR0_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn du_tr1_sel(&mut self) -> DU_TR1_SEL_W<DU_SEL_SPEC> {
        DU_TR1_SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn du_tr2_sel(&mut self) -> DU_TR2_SEL_W<DU_SEL_SPEC> {
        DU_TR2_SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
    #[inline(always)]
    #[must_use]
    pub fn du_data0_sel(&mut self) -> DU_DATA0_SEL_W<DU_SEL_SPEC> {
        DU_DATA0_SEL_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn du_data1_sel(&mut self) -> DU_DATA1_SEL_W<DU_SEL_SPEC> {
        DU_DATA1_SEL_W::new(self, 28)
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
#[doc = "Data unit component input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`du_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`du_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DU_SEL_SPEC;
impl crate::RegisterSpec for DU_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`du_sel::R`](R) reader structure"]
impl crate::Readable for DU_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`du_sel::W`](W) writer structure"]
impl crate::Writable for DU_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DU_SEL to value 0"]
impl crate::Resettable for DU_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `PORT_SEL1` reader"]
pub type R = crate::R<PORT_SEL1_SPEC>;
#[doc = "Register `PORT_SEL1` writer"]
pub type W = crate::W<PORT_SEL1_SPEC>;
#[doc = "Field `IO4_SEL` reader - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
pub type IO4_SEL_R = crate::FieldReader;
#[doc = "Field `IO4_SEL` writer - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
pub type IO4_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IO5_SEL` reader - Selects connection for IO pin 5 route."]
pub type IO5_SEL_R = crate::FieldReader;
#[doc = "Field `IO5_SEL` writer - Selects connection for IO pin 5 route."]
pub type IO5_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IO6_SEL` reader - Selects connection for IO pin 6 route."]
pub type IO6_SEL_R = crate::FieldReader;
#[doc = "Field `IO6_SEL` writer - Selects connection for IO pin 6 route."]
pub type IO6_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IO7_SEL` reader - Selects connection for IO pin 7 route."]
pub type IO7_SEL_R = crate::FieldReader;
#[doc = "Field `IO7_SEL` writer - Selects connection for IO pin 7 route."]
pub type IO7_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
    #[inline(always)]
    pub fn io4_sel(&self) -> IO4_SEL_R {
        IO4_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 5 route."]
    #[inline(always)]
    pub fn io5_sel(&self) -> IO5_SEL_R {
        IO5_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 6 route."]
    #[inline(always)]
    pub fn io6_sel(&self) -> IO6_SEL_R {
        IO6_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 7 route."]
    #[inline(always)]
    pub fn io7_sel(&self) -> IO7_SEL_R {
        IO7_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
    #[inline(always)]
    #[must_use]
    pub fn io4_sel(&mut self) -> IO4_SEL_W<PORT_SEL1_SPEC> {
        IO4_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 5 route."]
    #[inline(always)]
    #[must_use]
    pub fn io5_sel(&mut self) -> IO5_SEL_W<PORT_SEL1_SPEC> {
        IO5_SEL_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 6 route."]
    #[inline(always)]
    #[must_use]
    pub fn io6_sel(&mut self) -> IO6_SEL_W<PORT_SEL1_SPEC> {
        IO6_SEL_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 7 route."]
    #[inline(always)]
    #[must_use]
    pub fn io7_sel(&mut self) -> IO7_SEL_W<PORT_SEL1_SPEC> {
        IO7_SEL_W::new(self, 24)
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
#[doc = "Port selection 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`port_sel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`port_sel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PORT_SEL1_SPEC;
impl crate::RegisterSpec for PORT_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`port_sel1::R`](R) reader structure"]
impl crate::Readable for PORT_SEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`port_sel1::W`](W) writer structure"]
impl crate::Writable for PORT_SEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORT_SEL1 to value 0"]
impl crate::Resettable for PORT_SEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

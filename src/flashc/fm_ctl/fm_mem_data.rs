#[doc = "Register `FM_MEM_DATA[%s]` reader"]
pub type R = crate::R<FM_MEM_DATA_SPEC>;
#[doc = "Field `DATA32` reader - Sense amplifier and column multiplexer structure Bytes. The read data is dependent on FM_CTL.IF_SEL: - IF_SEL is 0: data as specified by the R interface address - IF_SEL is 1: data as specified by FM_MEM_ADDR and the offset of the accessed FM_MEM_DATA register."]
pub type DATA32_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Sense amplifier and column multiplexer structure Bytes. The read data is dependent on FM_CTL.IF_SEL: - IF_SEL is 0: data as specified by the R interface address - IF_SEL is 1: data as specified by FM_MEM_ADDR and the offset of the accessed FM_MEM_DATA register."]
    #[inline(always)]
    pub fn data32(&self) -> DATA32_R {
        DATA32_R::new(self.bits)
    }
}
#[doc = "Flash macro memory sense amplifier and column decoder data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fm_mem_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FM_MEM_DATA_SPEC;
impl crate::RegisterSpec for FM_MEM_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_mem_data::R`](R) reader structure"]
impl crate::Readable for FM_MEM_DATA_SPEC {}
#[doc = "`reset()` method sets FM_MEM_DATA[%s]
to value 0"]
impl crate::Resettable for FM_MEM_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

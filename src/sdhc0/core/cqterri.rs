#[doc = "Register `CQTERRI` reader"]
pub type R = crate::R<CQTERRI_SPEC>;
#[doc = "Field `RESP_ERR_CMD_INDX` reader - This field captures the index of the command that was executed on the command line when the error occurred."]
pub type RESP_ERR_CMD_INDX_R = crate::FieldReader;
#[doc = "Field `RESP_ERR_TASKID` reader - This field captures the ID of the task which was executed on the command line when the error occurred."]
pub type RESP_ERR_TASKID_R = crate::FieldReader;
#[doc = "Field `RESP_ERR_FIELDS_VALID` reader - This bit is updated when an error is detected while a command transaction was in progress. Values: - 0x1 (SET): Response-related error is detected. Check contents of RESP_ERR_TASKID and RESP_ERR_CMD_INDX fields - 0x0 (NOT_SET): Ignore contents of RESP_ERR_TASKID and RESP_ERR_CMD_INDX"]
pub type RESP_ERR_FIELDS_VALID_R = crate::BitReader;
#[doc = "Field `TRANS_ERR_CMD_INDX` reader - This field captures the index of the command that was executed and whose data transfer has errors."]
pub type TRANS_ERR_CMD_INDX_R = crate::FieldReader;
#[doc = "Field `TRANS_ERR_TASKID` reader - This field captures the ID of the task that was executed and whose data transfer has errors."]
pub type TRANS_ERR_TASKID_R = crate::FieldReader;
#[doc = "Field `TRANS_ERR_FIELDS_VALID` reader - This bit is updated when an error is detected while a data transfer transaction was in progress. Values: - 0x1 (SET): data transfer related error detected. Check contents of TRANS_ERR_TASKID and TRANS_ERR_CMD_INDX fields - 0x0 (NOT_SET): Ignore contents of TRANS_ERR_TASKID and TRANS_ERR_CMD_INDX"]
pub type TRANS_ERR_FIELDS_VALID_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - This field captures the index of the command that was executed on the command line when the error occurred."]
    #[inline(always)]
    pub fn resp_err_cmd_indx(&self) -> RESP_ERR_CMD_INDX_R {
        RESP_ERR_CMD_INDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - This field captures the ID of the task which was executed on the command line when the error occurred."]
    #[inline(always)]
    pub fn resp_err_taskid(&self) -> RESP_ERR_TASKID_R {
        RESP_ERR_TASKID_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - This bit is updated when an error is detected while a command transaction was in progress. Values: - 0x1 (SET): Response-related error is detected. Check contents of RESP_ERR_TASKID and RESP_ERR_CMD_INDX fields - 0x0 (NOT_SET): Ignore contents of RESP_ERR_TASKID and RESP_ERR_CMD_INDX"]
    #[inline(always)]
    pub fn resp_err_fields_valid(&self) -> RESP_ERR_FIELDS_VALID_R {
        RESP_ERR_FIELDS_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - This field captures the index of the command that was executed and whose data transfer has errors."]
    #[inline(always)]
    pub fn trans_err_cmd_indx(&self) -> TRANS_ERR_CMD_INDX_R {
        TRANS_ERR_CMD_INDX_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - This field captures the ID of the task that was executed and whose data transfer has errors."]
    #[inline(always)]
    pub fn trans_err_taskid(&self) -> TRANS_ERR_TASKID_R {
        TRANS_ERR_TASKID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - This bit is updated when an error is detected while a data transfer transaction was in progress. Values: - 0x1 (SET): data transfer related error detected. Check contents of TRANS_ERR_TASKID and TRANS_ERR_CMD_INDX fields - 0x0 (NOT_SET): Ignore contents of TRANS_ERR_TASKID and TRANS_ERR_CMD_INDX"]
    #[inline(always)]
    pub fn trans_err_fields_valid(&self) -> TRANS_ERR_FIELDS_VALID_R {
        TRANS_ERR_FIELDS_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "CQ Task Error Information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqterri::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CQTERRI_SPEC;
impl crate::RegisterSpec for CQTERRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqterri::R`](R) reader structure"]
impl crate::Readable for CQTERRI_SPEC {}
#[doc = "`reset()` method sets CQTERRI to value 0"]
impl crate::Resettable for CQTERRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

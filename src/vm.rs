

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpCode {
    CSTR, CKEY, CINT, CSHORT, CFLOAT, CBOOL, CNIL,CTYPE,
    NSSETS, NSGETS,
    ADDVV, SUBVV, MULVV, DIVVV, MODVV, POWVV,
    ISLT, ISGE, ISLE, ISGT, ISEQ, ISNEQ,
    MOV, NOT, NEG,
    JUMP, JUMPF, JUMPT,
    CALL, RET,
    APPLY,
    FNEW, VFNEW,
    DROP, TRANC, UCLO,
    GETFREEVAR,
    LOOP, BULKMOV,
    NEWARRAY, GETARRAY, SETARRAY,
    ALLOC, SETFIELD, GETFIELD,
    FUNCF, FUNCV,
    EXIT
}

pub enum InstrType {
    TyABC,
    TyAD
}

pub type Keyword = String;

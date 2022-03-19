
pub struct Command
{
    pub func: FuncTypes,
    pub arg: Vec<u8>
}

pub enum FuncTypes
{
    BIT,
    CRM,
    CMP,
    DIF,
    DUP,
    END,
    PED,
    LOP,
    PRT,
    PCT,
    QUT,
    RED,
    REM,
    SKP,
    SUB,
    SUM,
    SWP
}
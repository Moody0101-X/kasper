// ENUMS AND CONSTANTS.
// USED BY THE PARSER FOR LOGIC MAPPING.
//
pub const UMAX_8_BIT:   u8    = (0xFF);
pub const UMAX_16_BIT:  u16   = (0xFFFF);
pub const UMAX_32_BIT:  u32   = (0xFFFFFFFF);
pub const UMAX_64_BIT:  u64   = (0xFFFFFFFFFFFFFFFF);
pub const UMAX_128_BIT: u128  = (0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);

pub const MAX_8_BIT:   i8    = (UMAX_8_BIT   >> 1);
pub const MAX_16_BIT:  i16   = (UMAX_16_BIT  >> 1);
pub const MAX_32_BIT:  i32   = (UMAX_32_BIT  >> 1);
pub const MAX_64_BIT:  i64   = (UMAX_64_BIT  >> 1);
pub const MAX_128_BIT: i128  = (UMAX_128_BIT >> 1); // >> Division by two.

pub const DQUOTE:     char  = '\"';
pub const SQUOTE:     char  = '\'';
pub const SPACE:      char  = ' ';
pub const NL:         char  = '\n';
pub const OPAR:       char  = '(';
pub const CPAR:       char  = ')';
pub const OCURLY:     char  = '{';
pub const CCURLY:     char  = '}';
pub const PLUS:       char  = '+';
pub const MULT:       char  = '*';
pub const MINUS:      char  = '-';
pub const COMA:       char  = ',';
pub const SEMICOLON:  char  = ';';
pub const EQUAL:      char  = '=';
pub const GT:         char  = '>';
pub const LT:         char  = '<';
pub const QM:         char  = '!';
pub const COMMENT:    char  = '/';
pub const ESCAPE:     char  = '\\';
pub const PRINT:      &str  = "print";
pub const DEFINE:     &str  = "define";
pub const PROCC:      &str  = "process";
pub const THIN_ARROW: &str  = "->"; 
pub const FAT_ARROW:  &str  = "=>"; 
// TYPLES:
pub const STRING:     &str  = "string"; 
pub const INT:        &str  = "int";
pub const INT_64:        &str  = "i64";

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum TokenT {
    // Special tokens.
    COMMENT__,
    DQUOTE__,
    SQUOTE__,
    OPAR__,
    CPAR__,
    OCURLY__,
    CCURLY__,
    PLUS__,
    MINUS__,
    MULT__,
    COMA__,
    SEMICOLON__,
    EQUAL__,
    GT__,
    LT__,
    QM__,
    THIN_ARROW__,
    FAT_ARROW__,

    // Other
    NONE__,
    NUMBER__,
    STRING__,
    VARNAME__,

    // Types
    INT_T,
    INT_T_64,
    STRING_T,

    // Built-ins + funcs..
    FUNC_CALL__,
    PRINT__,
    
    // Key words.
    KEY_WORD_DEFINE__,
    KEY_WORD_PROCESS__,
}

impl fmt::Display for TokenT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            TokenT::NONE__             => "NONE__",
            TokenT::DQUOTE__           => "DQUOTE__",
            TokenT::SQUOTE__           => "SQUOTE__",
            TokenT::OPAR__             => "OPAR__",
            TokenT::CPAR__             => "CPAR__",
            TokenT::OCURLY__           => "OCURLY__",
            TokenT::CCURLY__           => "CCURLY__",
            TokenT::PLUS__             => "PLUS__",
            TokenT::MINUS__            => "MINUS__",
            TokenT::COMA__             => "COMA__",
            TokenT::SEMICOLON__        => "SEMICOLON__",
            TokenT::EQUAL__            => "EQUAL__",
            TokenT::GT__               => "GT__",
            TokenT::LT__               => "LT__",
            TokenT::NUMBER__           => "NUMBER__",
            TokenT::STRING__           => "STRING__",
            TokenT::QM__               => "QM__",
            TokenT::VARNAME__          => "VARNAME__",
            TokenT::COMMENT__          => "COMMENT__",
            TokenT::PRINT__            => "PRINT__",
            TokenT::KEY_WORD_DEFINE__  => "KEY_WORD_DEFINE__",
            TokenT::KEY_WORD_PROCESS__ => "KEY_WORD_PROCESS__", 
            TokenT::THIN_ARROW__       => "THIN_ARROW__",
            TokenT::FAT_ARROW__        => "FAT_ARROW__",
            TokenT::FUNC_CALL__        => "FUNC_CALL__",
            TokenT::STRING_T           => "STRING_T",
            TokenT::INT_T              => "INT_T",
            TokenT::INT_T_64           => "INT_64",
            TokenT::MULT__             => "MULT__",
        }; 
     
        write!(f, "{}", printable)
    }
}

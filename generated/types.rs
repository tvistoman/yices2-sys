/* automatically generated by rust-bindgen */

pub type size_t = ::std::os::raw::c_ulonglong;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}
#[test]
fn bindgen_test_layout___crt_locale_pointers() {
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).locinfo as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(locinfo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__crt_locale_pointers>())).mbcinfo as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(mbcinfo)
        )
    );
}
pub type _locale_t = *mut __crt_locale_pointers;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout__Mbstatet() {
    assert_eq!(
        ::std::mem::size_of::<_Mbstatet>(),
        8usize,
        concat!("Size of: ", stringify!(_Mbstatet))
    );
    assert_eq!(
        ::std::mem::align_of::<_Mbstatet>(),
        4usize,
        concat!("Alignment of ", stringify!(_Mbstatet))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Wchar as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Wchar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._Byte as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Byte)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_Mbstatet>()))._State as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_State)
        )
    );
}
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = size_t;
pub type fpos_t = ::std::os::raw::c_longlong;
pub type term_t = i32;
pub type type_t = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct context_s {
    _unused: [u8; 0],
}
pub type context_t = context_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct model_s {
    _unused: [u8; 0],
}
pub type model_t = model_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ctx_config_s {
    _unused: [u8; 0],
}
pub type ctx_config_t = ctx_config_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct param_s {
    _unused: [u8; 0],
}
pub type param_t = param_s;
pub const STATUS_IDLE: smt_status = 0;
pub const STATUS_SEARCHING: smt_status = 1;
pub const STATUS_UNKNOWN: smt_status = 2;
pub const STATUS_SAT: smt_status = 3;
pub const STATUS_UNSAT: smt_status = 4;
pub const STATUS_INTERRUPTED: smt_status = 5;
pub const STATUS_ERROR: smt_status = 6;
pub type smt_status = i32;
pub use self::smt_status as smt_status_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct term_vector_s {
    pub capacity: u32,
    pub size: u32,
    pub data: *mut term_t,
}
#[test]
fn bindgen_test_layout_term_vector_s() {
    assert_eq!(
        ::std::mem::size_of::<term_vector_s>(),
        16usize,
        concat!("Size of: ", stringify!(term_vector_s))
    );
    assert_eq!(
        ::std::mem::align_of::<term_vector_s>(),
        8usize,
        concat!("Alignment of ", stringify!(term_vector_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<term_vector_s>())).capacity as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(term_vector_s),
            "::",
            stringify!(capacity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<term_vector_s>())).size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(term_vector_s),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<term_vector_s>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(term_vector_s),
            "::",
            stringify!(data)
        )
    );
}
pub type term_vector_t = term_vector_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct type_vector_s {
    pub capacity: u32,
    pub size: u32,
    pub data: *mut type_t,
}
#[test]
fn bindgen_test_layout_type_vector_s() {
    assert_eq!(
        ::std::mem::size_of::<type_vector_s>(),
        16usize,
        concat!("Size of: ", stringify!(type_vector_s))
    );
    assert_eq!(
        ::std::mem::align_of::<type_vector_s>(),
        8usize,
        concat!("Alignment of ", stringify!(type_vector_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<type_vector_s>())).capacity as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(type_vector_s),
            "::",
            stringify!(capacity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<type_vector_s>())).size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(type_vector_s),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<type_vector_s>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(type_vector_s),
            "::",
            stringify!(data)
        )
    );
}
pub type type_vector_t = type_vector_s;
pub const YICES_CONSTRUCTOR_ERROR: term_constructor = -1;
pub const YICES_BOOL_CONSTANT: term_constructor = 0;
pub const YICES_ARITH_CONSTANT: term_constructor = 1;
pub const YICES_BV_CONSTANT: term_constructor = 2;
pub const YICES_SCALAR_CONSTANT: term_constructor = 3;
pub const YICES_VARIABLE: term_constructor = 4;
pub const YICES_UNINTERPRETED_TERM: term_constructor = 5;
pub const YICES_ITE_TERM: term_constructor = 6;
pub const YICES_APP_TERM: term_constructor = 7;
pub const YICES_UPDATE_TERM: term_constructor = 8;
pub const YICES_TUPLE_TERM: term_constructor = 9;
pub const YICES_EQ_TERM: term_constructor = 10;
pub const YICES_DISTINCT_TERM: term_constructor = 11;
pub const YICES_FORALL_TERM: term_constructor = 12;
pub const YICES_LAMBDA_TERM: term_constructor = 13;
pub const YICES_NOT_TERM: term_constructor = 14;
pub const YICES_OR_TERM: term_constructor = 15;
pub const YICES_XOR_TERM: term_constructor = 16;
pub const YICES_BV_ARRAY: term_constructor = 17;
pub const YICES_BV_DIV: term_constructor = 18;
pub const YICES_BV_REM: term_constructor = 19;
pub const YICES_BV_SDIV: term_constructor = 20;
pub const YICES_BV_SREM: term_constructor = 21;
pub const YICES_BV_SMOD: term_constructor = 22;
pub const YICES_BV_SHL: term_constructor = 23;
pub const YICES_BV_LSHR: term_constructor = 24;
pub const YICES_BV_ASHR: term_constructor = 25;
pub const YICES_BV_GE_ATOM: term_constructor = 26;
pub const YICES_BV_SGE_ATOM: term_constructor = 27;
pub const YICES_ARITH_GE_ATOM: term_constructor = 28;
pub const YICES_ARITH_ROOT_ATOM: term_constructor = 29;
pub const YICES_ABS: term_constructor = 30;
pub const YICES_CEIL: term_constructor = 31;
pub const YICES_FLOOR: term_constructor = 32;
pub const YICES_RDIV: term_constructor = 33;
pub const YICES_IDIV: term_constructor = 34;
pub const YICES_IMOD: term_constructor = 35;
pub const YICES_IS_INT_ATOM: term_constructor = 36;
pub const YICES_DIVIDES_ATOM: term_constructor = 37;
pub const YICES_SELECT_TERM: term_constructor = 38;
pub const YICES_BIT_TERM: term_constructor = 39;
pub const YICES_BV_SUM: term_constructor = 40;
pub const YICES_ARITH_SUM: term_constructor = 41;
pub const YICES_POWER_PRODUCT: term_constructor = 42;
pub type term_constructor = i32;
pub use self::term_constructor as term_constructor_t;
pub const YVAL_UNKNOWN: yval_tag = 0;
pub const YVAL_BOOL: yval_tag = 1;
pub const YVAL_RATIONAL: yval_tag = 2;
pub const YVAL_ALGEBRAIC: yval_tag = 3;
pub const YVAL_BV: yval_tag = 4;
pub const YVAL_SCALAR: yval_tag = 5;
pub const YVAL_TUPLE: yval_tag = 6;
pub const YVAL_FUNCTION: yval_tag = 7;
pub const YVAL_MAPPING: yval_tag = 8;
pub type yval_tag = i32;
pub use self::yval_tag as yval_tag_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yval_s {
    pub node_id: i32,
    pub node_tag: yval_tag_t,
}
#[test]
fn bindgen_test_layout_yval_s() {
    assert_eq!(
        ::std::mem::size_of::<yval_s>(),
        8usize,
        concat!("Size of: ", stringify!(yval_s))
    );
    assert_eq!(
        ::std::mem::align_of::<yval_s>(),
        4usize,
        concat!("Alignment of ", stringify!(yval_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<yval_s>())).node_id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(yval_s),
            "::",
            stringify!(node_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<yval_s>())).node_tag as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(yval_s),
            "::",
            stringify!(node_tag)
        )
    );
}
pub type yval_t = yval_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yval_vector_s {
    pub capacity: u32,
    pub size: u32,
    pub data: *mut yval_t,
}
#[test]
fn bindgen_test_layout_yval_vector_s() {
    assert_eq!(
        ::std::mem::size_of::<yval_vector_s>(),
        16usize,
        concat!("Size of: ", stringify!(yval_vector_s))
    );
    assert_eq!(
        ::std::mem::align_of::<yval_vector_s>(),
        8usize,
        concat!("Alignment of ", stringify!(yval_vector_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<yval_vector_s>())).capacity as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(yval_vector_s),
            "::",
            stringify!(capacity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<yval_vector_s>())).size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(yval_vector_s),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<yval_vector_s>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(yval_vector_s),
            "::",
            stringify!(data)
        )
    );
}
pub type yval_vector_t = yval_vector_s;
pub const YICES_GEN_DEFAULT: yices_gen_mode = 0;
pub const YICES_GEN_BY_SUBST: yices_gen_mode = 1;
pub const YICES_GEN_BY_PROJ: yices_gen_mode = 2;
pub type yices_gen_mode = i32;
pub use self::yices_gen_mode as yices_gen_mode_t;
pub const NO_ERROR: error_code = 0;
pub const INVALID_TYPE: error_code = 1;
pub const INVALID_TERM: error_code = 2;
pub const INVALID_CONSTANT_INDEX: error_code = 3;
pub const INVALID_VAR_INDEX: error_code = 4;
pub const INVALID_TUPLE_INDEX: error_code = 5;
pub const INVALID_RATIONAL_FORMAT: error_code = 6;
pub const INVALID_FLOAT_FORMAT: error_code = 7;
pub const INVALID_BVBIN_FORMAT: error_code = 8;
pub const INVALID_BVHEX_FORMAT: error_code = 9;
pub const INVALID_BITSHIFT: error_code = 10;
pub const INVALID_BVEXTRACT: error_code = 11;
pub const INVALID_BITEXTRACT: error_code = 12;
pub const TOO_MANY_ARGUMENTS: error_code = 13;
pub const TOO_MANY_VARS: error_code = 14;
pub const MAX_BVSIZE_EXCEEDED: error_code = 15;
pub const DEGREE_OVERFLOW: error_code = 16;
pub const DIVISION_BY_ZERO: error_code = 17;
pub const POS_INT_REQUIRED: error_code = 18;
pub const NONNEG_INT_REQUIRED: error_code = 19;
pub const SCALAR_OR_UTYPE_REQUIRED: error_code = 20;
pub const FUNCTION_REQUIRED: error_code = 21;
pub const TUPLE_REQUIRED: error_code = 22;
pub const VARIABLE_REQUIRED: error_code = 23;
pub const ARITHTERM_REQUIRED: error_code = 24;
pub const BITVECTOR_REQUIRED: error_code = 25;
pub const SCALAR_TERM_REQUIRED: error_code = 26;
pub const WRONG_NUMBER_OF_ARGUMENTS: error_code = 27;
pub const TYPE_MISMATCH: error_code = 28;
pub const INCOMPATIBLE_TYPES: error_code = 29;
pub const DUPLICATE_VARIABLE: error_code = 30;
pub const INCOMPATIBLE_BVSIZES: error_code = 31;
pub const EMPTY_BITVECTOR: error_code = 32;
pub const ARITHCONSTANT_REQUIRED: error_code = 33;
pub const INVALID_MACRO: error_code = 34;
pub const TOO_MANY_MACRO_PARAMS: error_code = 35;
pub const TYPE_VAR_REQUIRED: error_code = 36;
pub const DUPLICATE_TYPE_VAR: error_code = 37;
pub const BVTYPE_REQUIRED: error_code = 38;
pub const BAD_TERM_DECREF: error_code = 39;
pub const BAD_TYPE_DECREF: error_code = 40;
pub const INVALID_TYPE_OP: error_code = 41;
pub const INVALID_TERM_OP: error_code = 42;
pub const INVALID_TOKEN: error_code = 100;
pub const SYNTAX_ERROR: error_code = 101;
pub const UNDEFINED_TYPE_NAME: error_code = 102;
pub const UNDEFINED_TERM_NAME: error_code = 103;
pub const REDEFINED_TYPE_NAME: error_code = 104;
pub const REDEFINED_TERM_NAME: error_code = 105;
pub const DUPLICATE_NAME_IN_SCALAR: error_code = 106;
pub const DUPLICATE_VAR_NAME: error_code = 107;
pub const INTEGER_OVERFLOW: error_code = 108;
pub const INTEGER_REQUIRED: error_code = 109;
pub const RATIONAL_REQUIRED: error_code = 110;
pub const SYMBOL_REQUIRED: error_code = 111;
pub const TYPE_REQUIRED: error_code = 112;
pub const NON_CONSTANT_DIVISOR: error_code = 113;
pub const NEGATIVE_BVSIZE: error_code = 114;
pub const INVALID_BVCONSTANT: error_code = 115;
pub const TYPE_MISMATCH_IN_DEF: error_code = 116;
pub const ARITH_ERROR: error_code = 117;
pub const BVARITH_ERROR: error_code = 118;
pub const CTX_FREE_VAR_IN_FORMULA: error_code = 300;
pub const CTX_LOGIC_NOT_SUPPORTED: error_code = 301;
pub const CTX_UF_NOT_SUPPORTED: error_code = 302;
pub const CTX_ARITH_NOT_SUPPORTED: error_code = 303;
pub const CTX_BV_NOT_SUPPORTED: error_code = 304;
pub const CTX_ARRAYS_NOT_SUPPORTED: error_code = 305;
pub const CTX_QUANTIFIERS_NOT_SUPPORTED: error_code = 306;
pub const CTX_LAMBDAS_NOT_SUPPORTED: error_code = 307;
pub const CTX_NONLINEAR_ARITH_NOT_SUPPORTED: error_code = 308;
pub const CTX_FORMULA_NOT_IDL: error_code = 309;
pub const CTX_FORMULA_NOT_RDL: error_code = 310;
pub const CTX_TOO_MANY_ARITH_VARS: error_code = 311;
pub const CTX_TOO_MANY_ARITH_ATOMS: error_code = 312;
pub const CTX_TOO_MANY_BV_VARS: error_code = 313;
pub const CTX_TOO_MANY_BV_ATOMS: error_code = 314;
pub const CTX_ARITH_SOLVER_EXCEPTION: error_code = 315;
pub const CTX_BV_SOLVER_EXCEPTION: error_code = 316;
pub const CTX_ARRAY_SOLVER_EXCEPTION: error_code = 317;
pub const CTX_SCALAR_NOT_SUPPORTED: error_code = 318;
pub const CTX_TUPLE_NOT_SUPPORTED: error_code = 319;
pub const CTX_UTYPE_NOT_SUPPORTED: error_code = 320;
pub const CTX_INVALID_OPERATION: error_code = 400;
pub const CTX_OPERATION_NOT_SUPPORTED: error_code = 401;
pub const CTX_UNKNOWN_DELEGATE: error_code = 420;
pub const CTX_DELEGATE_NOT_AVAILABLE: error_code = 421;
pub const CTX_INVALID_CONFIG: error_code = 500;
pub const CTX_UNKNOWN_PARAMETER: error_code = 501;
pub const CTX_INVALID_PARAMETER_VALUE: error_code = 502;
pub const CTX_UNKNOWN_LOGIC: error_code = 503;
pub const EVAL_UNKNOWN_TERM: error_code = 600;
pub const EVAL_FREEVAR_IN_TERM: error_code = 601;
pub const EVAL_QUANTIFIER: error_code = 602;
pub const EVAL_LAMBDA: error_code = 603;
pub const EVAL_OVERFLOW: error_code = 604;
pub const EVAL_FAILED: error_code = 605;
pub const EVAL_CONVERSION_FAILED: error_code = 606;
pub const EVAL_NO_IMPLICANT: error_code = 607;
pub const EVAL_NOT_SUPPORTED: error_code = 608;
pub const MDL_UNINT_REQUIRED: error_code = 700;
pub const MDL_CONSTANT_REQUIRED: error_code = 701;
pub const MDL_DUPLICATE_VAR: error_code = 702;
pub const MDL_FTYPE_NOT_ALLOWED: error_code = 703;
pub const MDL_CONSTRUCTION_FAILED: error_code = 704;
pub const YVAL_INVALID_OP: error_code = 800;
pub const YVAL_OVERFLOW: error_code = 801;
pub const YVAL_NOT_SUPPORTED: error_code = 802;
pub const MDL_GEN_TYPE_NOT_SUPPORTED: error_code = 900;
pub const MDL_GEN_NONLINEAR: error_code = 901;
pub const MDL_GEN_FAILED: error_code = 902;
pub const MCSAT_ERROR_UNSUPPORTED_THEORY: error_code = 1000;
pub const MCSAT_ERROR_NAMED_TERMS_NOT_SUPPORTED: error_code = 1001;
pub const OUTPUT_ERROR: error_code = 9000;
pub const INTERNAL_EXCEPTION: error_code = 9999;
pub type error_code = i32;
pub use self::error_code as error_code_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct error_report_s {
    pub code: error_code_t,
    pub line: u32,
    pub column: u32,
    pub term1: term_t,
    pub type1: type_t,
    pub term2: term_t,
    pub type2: type_t,
    pub badval: i64,
}
#[test]
fn bindgen_test_layout_error_report_s() {
    assert_eq!(
        ::std::mem::size_of::<error_report_s>(),
        40usize,
        concat!("Size of: ", stringify!(error_report_s))
    );
    assert_eq!(
        ::std::mem::align_of::<error_report_s>(),
        8usize,
        concat!("Alignment of ", stringify!(error_report_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<error_report_s>())).code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(error_report_s),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<error_report_s>())).line as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(error_report_s),
            "::",
            stringify!(line)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<error_report_s>())).column as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(error_report_s),
            "::",
            stringify!(column)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<error_report_s>())).term1 as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(error_report_s),
            "::",
            stringify!(term1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<error_report_s>())).type1 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(error_report_s),
            "::",
            stringify!(type1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<error_report_s>())).term2 as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(error_report_s),
            "::",
            stringify!(term2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<error_report_s>())).type2 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(error_report_s),
            "::",
            stringify!(type2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<error_report_s>())).badval as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(error_report_s),
            "::",
            stringify!(badval)
        )
    );
}
pub type error_report_t = error_report_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct interpolation_context_s {
    pub ctx_A: *mut context_t,
    pub ctx_B: *mut context_t,
    pub interpolant: term_t,
    pub model: *mut model_t,
}
#[test]
fn bindgen_test_layout_interpolation_context_s() {
    assert_eq!(
        ::std::mem::size_of::<interpolation_context_s>(),
        32usize,
        concat!("Size of: ", stringify!(interpolation_context_s))
    );
    assert_eq!(
        ::std::mem::align_of::<interpolation_context_s>(),
        8usize,
        concat!("Alignment of ", stringify!(interpolation_context_s))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<interpolation_context_s>())).ctx_A as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(interpolation_context_s),
            "::",
            stringify!(ctx_A)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<interpolation_context_s>())).ctx_B as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(interpolation_context_s),
            "::",
            stringify!(ctx_B)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<interpolation_context_s>())).interpolant as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(interpolation_context_s),
            "::",
            stringify!(interpolant)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<interpolation_context_s>())).model as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(interpolation_context_s),
            "::",
            stringify!(model)
        )
    );
}
pub type interpolation_context_t = interpolation_context_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}

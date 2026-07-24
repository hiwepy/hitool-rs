//! Hutool-aligned HTTP metadata and explicitly owned default headers.

use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};

/// Hutool-compatible HTTP status-code namespace.
#[derive(Debug, Clone, Copy, Default)]
pub struct HttpStatus;

impl HttpStatus {
    status_codes! {
        HTTP_CONTINUE = 100,
        HTTP_SWITCHING_PROTOCOLS = 101,
        HTTP_PROCESSING = 102,
        HTTP_CHECKPOINT = 103,
        HTTP_OK = 200,
        HTTP_CREATED = 201,
        HTTP_ACCEPTED = 202,
        HTTP_NOT_AUTHORITATIVE = 203,
        HTTP_NO_CONTENT = 204,
        HTTP_RESET = 205,
        HTTP_PARTIAL = 206,
        HTTP_MULTI_STATUS = 207,
        HTTP_ALREADY_REPORTED = 208,
        HTTP_IM_USED = 226,
        HTTP_MULT_CHOICE = 300,
        HTTP_MOVED_PERM = 301,
        HTTP_MOVED_TEMP = 302,
        HTTP_SEE_OTHER = 303,
        HTTP_NOT_MODIFIED = 304,
        HTTP_USE_PROXY = 305,
        HTTP_TEMP_REDIRECT = 307,
        HTTP_PERMANENT_REDIRECT = 308,
        HTTP_BAD_REQUEST = 400,
        HTTP_UNAUTHORIZED = 401,
        HTTP_PAYMENT_REQUIRED = 402,
        HTTP_FORBIDDEN = 403,
        HTTP_NOT_FOUND = 404,
        HTTP_BAD_METHOD = 405,
        HTTP_NOT_ACCEPTABLE = 406,
        HTTP_PROXY_AUTH = 407,
        HTTP_CLIENT_TIMEOUT = 408,
        HTTP_CONFLICT = 409,
        HTTP_GONE = 410,
        HTTP_LENGTH_REQUIRED = 411,
        HTTP_PRECON_FAILED = 412,
        HTTP_ENTITY_TOO_LARGE = 413,
        HTTP_REQ_TOO_LONG = 414,
        HTTP_UNSUPPORTED_TYPE = 415,
        HTTP_REQUESTED_RANGE_NOT_SATISFIABLE = 416,
        HTTP_EXPECTATION_FAILED = 417,
        HTTP_I_AM_A_TEAPOT = 418,
        HTTP_UNPROCESSABLE_ENTITY = 422,
        HTTP_LOCKED = 423,
        HTTP_FAILED_DEPENDENCY = 424,
        HTTP_TOO_EARLY = 425,
        HTTP_UPGRADE_REQUIRED = 426,
        HTTP_PRECONDITION_REQUIRED = 428,
        HTTP_TOO_MANY_REQUESTS = 429,
        HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE = 431,
        HTTP_UNAVAILABLE_FOR_LEGAL_REASONS = 451,
        HTTP_INTERNAL_ERROR = 500,
        HTTP_NOT_IMPLEMENTED = 501,
        HTTP_BAD_GATEWAY = 502,
        HTTP_UNAVAILABLE = 503,
        HTTP_GATEWAY_TIMEOUT = 504,
        HTTP_VERSION = 505,
        HTTP_VARIANT_ALSO_NEGOTIATES = 506,
        HTTP_INSUFFICIENT_STORAGE = 507,
        HTTP_LOOP_DETECTED = 508,
        HTTP_BANDWIDTH_LIMIT_EXCEEDED = 509,
        HTTP_NOT_EXTENDED = 510,
        HTTP_NETWORK_AUTHENTICATION_REQUIRED = 511,
    }

    /// Reports whether Hutool treats `response_code` as a redirect.
    #[must_use]
    pub const fn is_redirected(response_code: u16) -> bool {
        matches!(response_code, 301 | 302 | 303 | 307 | 308)
    }
}

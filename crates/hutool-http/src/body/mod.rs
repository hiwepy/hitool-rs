//! Request body helpers aligned with Hutool HTTP body package.

mod bytes_form;
mod multipart;
mod multipart_stream;

pub use bytes_form::{BytesBody, FormUrlEncodedBody, RequestBody, ResourceBody};
pub use multipart::MultipartBody;
pub use multipart_stream::MultipartOutputStream;

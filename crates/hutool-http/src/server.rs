//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use crate::HttpException;

/// 简单 HTTP 服务器，对齐 `cn.hutool.http.server.SimpleServer`。
///
/// Java 基于 `com.sun.net.httpserver.HttpServer`；Rust 推荐 axum/actix-web。
pub trait SimpleServer: Send + Sync {
    /// 对齐 `SimpleServer.addAction(String path, Action action)`
    fn add_action(&mut self, path: &str, action: Box<dyn Action>) -> Result<(), HttpException>;

    /// 对齐 `SimpleServer.addFilter(Filter filter)`
    fn add_filter(&mut self, filter: Box<dyn Filter>) -> Result<(), HttpException>;

    /// 对齐 `SimpleServer.setPort(int)`
    fn set_port(&mut self, port: u16) -> &mut Self;

    /// 对齐 `SimpleServer.getPort()`
    fn get_port(&self) -> u16;

    /// 对齐 `SimpleServer.start()`
    fn start(&self) -> Result<(), HttpException>;

    /// 对齐 `SimpleServer.stop()`
    fn stop(&self) -> Result<(), HttpException>;
}

/// HTTP Server 基类，对齐 `cn.hutool.http.server.HttpServerBase`。
pub trait HttpServerBase: Send + Sync {
    /// 对齐 `HttpServerBase.getServer()`
    fn get_server(&self) -> Option<Box<dyn Any>>;

    /// 对齐 `HttpServerBase.init(String host, int port)`
    fn init(&mut self, host: &str, port: u16) -> Result<(), HttpException>;
}

/// HTTP 服务器请求，对齐 `cn.hutool.http.server.HttpServerRequest`。
pub trait HttpServerRequest: Send + Sync {
    /// 对齐 `HttpServerRequest.getMethod()`
    fn get_method(&self) -> &str;

    /// 对齐 `HttpServerRequest.getURL()`
    fn get_url(&self) -> &str;

    /// 对齐 `HttpServerRequest.getPath()`
    fn get_path(&self) -> &str;

    /// 对齐 `HttpServerRequest.getQuery()`
    fn get_query(&self) -> &str;

    /// 对齐 `HttpServerRequest.getHeader(String)`
    fn get_header(&self, name: &str) -> Option<String>;

    /// 对齐 `HttpServerRequest.getHeaders()`
    fn get_headers(&self) -> HashMap<String, Vec<String>>;

    /// 对齐 `HttpServerRequest.getBody()`
    fn get_body(&self) -> Result<String, HttpException>;

    /// 对齐 `HttpServerRequest.getBodyBytes()`
    fn get_body_bytes(&self) -> Result<Vec<u8>, HttpException>;

    /// 对齐 `HttpServerRequest.getParam(String)`
    fn get_param(&self, name: &str) -> Option<String>;

    /// 对齐 `HttpServerRequest.getParamMap()`
    fn get_param_map(&self) -> HashMap<String, Vec<String>>;

    /// 对齐 `HttpServerRequest.getRemoteAddr()`
    fn get_remote_addr(&self) -> Option<SocketAddr>;

    /// 对齐 `HttpServerRequest.getRemoteClientIP()`
    fn get_remote_client_ip(&self) -> Option<String> {
        self.get_remote_addr().map(|a| a.ip().to_string())
    }

    /// 对齐 `HttpServerRequest.getCookieValue(String)`
    fn get_cookie_value(&self, name: &str) -> Option<String>;

    /// 对齐 `HttpServerRequest.isGetMethod()`
    fn is_get_method(&self) -> bool {
        self.get_method().eq_ignore_ascii_case("GET")
    }

    /// 对齐 `HttpServerRequest.isPostMethod()`
    fn is_post_method(&self) -> bool {
        self.get_method().eq_ignore_ascii_case("POST")
    }

    /// 对齐 `HttpServerRequest.isMultipart()`
    fn is_multipart(&self) -> bool {
        self.get_header("Content-Type")
            .map(|c| c.starts_with("multipart/"))
            .unwrap_or(false)
    }
}

/// HTTP 服务器响应，对齐 `cn.hutool.http.server.HttpServerResponse`。
pub trait HttpServerResponse: Send + Sync {
    /// 对齐 `HttpServerResponse.setStatus(int)`
    fn set_status(&mut self, status: u16);

    /// 对齐 `HttpServerResponse.getStatus()`
    fn get_status(&self) -> u16;

    /// 对齐 `HttpServerResponse.setHeader(String, String)`
    fn set_header(&mut self, name: &str, value: &str);

    /// 对齐 `HttpServerResponse.addHeader(String, String)`
    fn add_header(&mut self, name: &str, value: &str);

    /// 对齐 `HttpServerResponse.setHeaders(Map)`
    fn set_headers(&mut self, headers: HashMap<String, String>);

    /// 对齐 `HttpServerResponse.setCookie(String, String)` (简化版)
    fn set_cookie(&mut self, name: &str, value: &str);

    /// 对齐 `HttpServerResponse.setContentType(String)`
    fn set_content_type(&mut self, content_type: &str);

    /// 对齐 `HttpServerResponse.setContentLength(int)` / `setContentLengthLong(long)`
    fn set_content_length(&mut self, length: u64);

    /// 对齐 `HttpServerResponse.write(String)` / `write(byte[])`
    fn write(&mut self, body: &[u8]) -> Result<(), HttpException>;
}

/// HttpServerResponse 扩展 trait，提供链式 API（独立 trait 保持 dyn-compatibility）
pub trait HttpServerResponseExt: HttpServerResponse {
    /// 对齐 `HttpServerResponse.print(String)`
    fn print(&mut self, body: &str) -> Result<(), HttpException> {
        self.write(body.as_bytes())
    }

    /// 对齐 `HttpServerResponse.send(int, String)` 简化
    fn send(&mut self, status: u16, body: &str) -> Result<(), HttpException> {
        self.set_status(status);
        self.print(body)
    }
}

impl<T: HttpServerResponse + ?Sized> HttpServerResponseExt for T {}

/// HTTP 处理动作，对齐 `cn.hutool.http.server.action.Action`。
pub trait Action: Send + Sync {
    /// 对齐 `Action.doAction(HttpServerRequest, HttpServerResponse, Filter.Chain)`
    fn do_action(
        &self,
        req: &dyn HttpServerRequest,
        resp: &mut dyn HttpServerResponse,
    ) -> Result<(), HttpException>;
}

/// 根路径 Action，对齐 `cn.hutool.http.server.action.RootAction`。
///
/// 处理根路径 `/` 的默认 action。
pub struct RootAction;

impl Action for RootAction {
    fn do_action(
        &self,
        _req: &dyn HttpServerRequest,
        resp: &mut dyn HttpServerResponse,
    ) -> Result<(), HttpException> {
        resp.set_status(200);
        resp.write(b"Hello from HiTool RootAction")
    }
}

/// 过滤器，对齐 `cn.hutool.http.server.filter.Filter`。
pub trait Filter: Send + Sync {
    /// 对齐 `Filter.doFilter(HttpServerRequest, HttpServerResponse, Filter.Chain)`
    fn do_filter(
        &self,
        req: &dyn HttpServerRequest,
        resp: &mut dyn HttpServerResponse,
        chain: &dyn FilterChain,
    ) -> Result<(), HttpException>;
}

/// 过滤器链，对齐 `cn.hutool.http.server.filter.Filter.Chain`。
pub trait FilterChain: Send + Sync {
    /// 对齐 `Filter.Chain.doFilter(HttpServerRequest, HttpServerResponse)`
    fn do_filter(
        &self,
        req: &dyn HttpServerRequest,
        resp: &mut dyn HttpServerResponse,
    ) -> Result<(), HttpException>;
}

/// 默认异常过滤器，对齐 `cn.hutool.http.server.filter.DefaultExceptionFilter`。
///
/// 捕获异常并返回 500。
pub struct DefaultExceptionFilter;

impl Filter for DefaultExceptionFilter {
    fn do_filter(
        &self,
        req: &dyn HttpServerRequest,
        resp: &mut dyn HttpServerResponse,
        chain: &dyn FilterChain,
    ) -> Result<(), HttpException> {
        // 先走 chain，出错则改 status
        match chain.do_filter(req, resp) {
            Ok(()) => Ok(()),
            Err(e) => {
                resp.set_status(500);
                let _ = resp.write(e.to_string().as_bytes());
                Ok(())
            }
        }
    }
}

/// HttpExchange 包装器，对齐 `cn.hutool.http.server.HttpExchangeWrapper`。
///
/// 依赖 `com.sun.net.httpserver.HttpExchange`，unsafe-to-copy。
pub trait HttpExchangeWrapper: Send + Sync {
    /// 对齐 `HttpExchangeWrapper.getExchange()`
    fn get_exchange(&self) -> Option<Box<dyn Any>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StubRequest {
        method: String,
        path: String,
        headers: HashMap<String, Vec<String>>,
    }

    impl HttpServerRequest for StubRequest {
        fn get_method(&self) -> &str {
            &self.method
        }
        fn get_url(&self) -> &str {
            &self.path
        }
        fn get_path(&self) -> &str {
            &self.path
        }
        fn get_query(&self) -> &str {
            ""
        }
        fn get_header(&self, name: &str) -> Option<String> {
            self.headers
                .get(&name.to_lowercase())
                .and_then(|vs| vs.first().cloned())
        }
        fn get_headers(&self) -> HashMap<String, Vec<String>> {
            self.headers.clone()
        }
        fn get_body(&self) -> Result<String, HttpException> {
            Ok("".into())
        }
        fn get_body_bytes(&self) -> Result<Vec<u8>, HttpException> {
            Ok(vec![])
        }
        fn get_param(&self, _name: &str) -> Option<String> {
            None
        }
        fn get_param_map(&self) -> HashMap<String, Vec<String>> {
            HashMap::new()
        }
        fn get_remote_addr(&self) -> Option<SocketAddr> {
            None
        }
        fn get_cookie_value(&self, _name: &str) -> Option<String> {
            None
        }
    }

    #[test]
    fn test_request_method_predicates() {
        let mut headers = HashMap::new();
        headers.insert("content-type".into(), vec!["multipart/form-data".into()]);
        let req = StubRequest {
            method: "POST".into(),
            path: "/".into(),
            headers,
        };
        assert!(req.is_post_method());
        assert!(!req.is_get_method());
        assert!(req.is_multipart());
    }

    #[test]
    fn test_root_action() {
        struct StubResponse;
        impl HttpServerResponse for StubResponse {
            fn set_status(&mut self, _s: u16) {}
            fn get_status(&self) -> u16 {
                200
            }
            fn set_header(&mut self, _n: &str, _v: &str) {}
            fn add_header(&mut self, _n: &str, _v: &str) {}
            fn set_headers(&mut self, _h: HashMap<String, String>) {}
            fn set_cookie(&mut self, _n: &str, _v: &str) {}
            fn set_content_type(&mut self, _c: &str) {}
            fn set_content_length(&mut self, _l: u64) {}
            fn write(&mut self, _b: &[u8]) -> Result<(), HttpException> {
                Ok(())
            }
        }
        let req = StubRequest {
            method: "GET".into(),
            path: "/".into(),
            headers: HashMap::new(),
        };
        let mut resp = StubResponse;
        let action = RootAction;
        let r = action.do_action(&req, &mut resp);
        assert!(r.is_ok());
    }
}
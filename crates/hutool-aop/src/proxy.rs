//! Proxy factories and the Hutool-aligned `ProxyUtil` facade.

use crate::{
    HandlerProxy, Method,
    aspects::Aspect,
    interceptor::{CglibInterceptor, JdkInterceptor, SpringCglibInterceptor},
};
use std::{fmt, sync::Arc};

/// Available explicit proxy strategies.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ProxyBackend {
    /// JDK callback ordering.
    #[default]
    Jdk,
    /// CGLIB callback ordering.
    Cglib,
    /// Spring's repackaged CGLIB callback ordering.
    SpringCglib,
}

/// A typed proxy produced by a [`ProxyFactory`].
pub enum Proxy<T, A, R, E> {
    /// JDK-style proxy.
    Jdk(JdkInterceptor<T, A, R, E>),
    /// CGLIB-style proxy.
    Cglib(CglibInterceptor<T, A, R, E>),
    /// Spring-CGLIB-style proxy.
    SpringCglib(SpringCglibInterceptor<T, A, R, E>),
}

impl<T: fmt::Debug, A, R, E> fmt::Debug for Proxy<T, A, R, E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Jdk(proxy) => proxy.fmt(formatter),
            Self::Cglib(proxy) => proxy.fmt(formatter),
            Self::SpringCglib(proxy) => proxy.fmt(formatter),
        }
    }
}

impl<T, A, R, E> Proxy<T, A, R, E> {
    /// Returns the proxied target.
    #[must_use]
    pub fn get_target(&self) -> &T {
        match self {
            Self::Jdk(proxy) => proxy.get_target(),
            Self::Cglib(proxy) => proxy.get_target(),
            Self::SpringCglib(proxy) => proxy.get_target(),
        }
    }

    /// Returns the mutable proxied target.
    #[must_use]
    pub fn get_target_mut(&mut self) -> &mut T {
        match self {
            Self::Jdk(proxy) => proxy.get_target_mut(),
            Self::Cglib(proxy) => proxy.get_target_mut(),
            Self::SpringCglib(proxy) => proxy.get_target_mut(),
        }
    }

    /// Consumes the proxy and returns its target.
    #[must_use]
    pub fn into_target(self) -> T {
        match self {
            Self::Jdk(proxy) => proxy.into_target(),
            Self::Cglib(proxy) => proxy.into_target(),
            Self::SpringCglib(proxy) => proxy.into_target(),
        }
    }

    /// Invokes an operation through the selected proxy strategy.
    pub fn invoke<F>(&mut self, method: &Method, args: &mut A, operation: F) -> Result<Option<R>, E>
    where
        F: FnOnce(&mut T, &mut A) -> Result<R, E>,
    {
        match self {
            Self::Jdk(proxy) => proxy.invoke(method, args, operation),
            Self::Cglib(proxy) => proxy.invoke(method, args, operation),
            Self::SpringCglib(proxy) => proxy.invoke(method, args, operation),
        }
    }
}

/// Selects and constructs typed aspect proxies.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ProxyFactory {
    backend: ProxyBackend,
}

impl ProxyFactory {
    /// Creates the default JDK-style factory.
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// Creates a factory for an explicit backend.
    #[must_use]
    pub const fn with_backend(backend: ProxyBackend) -> Self {
        Self { backend }
    }

    /// Returns the selected backend.
    #[must_use]
    pub const fn backend(self) -> ProxyBackend {
        self.backend
    }

    /// Creates a proxy with an owned aspect.
    pub fn proxy<T, A, R, E, I>(self, target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        self.proxy_shared(target, Arc::new(aspect))
    }

    /// Creates a proxy by default-constructing its aspect type.
    pub fn proxy_default<T, A, R, E, I>(self, target: T) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + Default + 'static,
    {
        self.proxy(target, I::default())
    }

    /// Creates a proxy with a shared aspect.
    pub fn proxy_shared<T, A, R, E>(
        self,
        target: T,
        aspect: Arc<dyn Aspect<T, A, R, E>>,
    ) -> Proxy<T, A, R, E> {
        match self.backend {
            ProxyBackend::Jdk => Proxy::Jdk(JdkInterceptor::with_shared(target, aspect)),
            ProxyBackend::Cglib => Proxy::Cglib(CglibInterceptor::with_shared(target, aspect)),
            ProxyBackend::SpringCglib => {
                Proxy::SpringCglib(SpringCglibInterceptor::with_shared(target, aspect))
            }
        }
    }

    /// Creates a default JDK-style proxy.
    pub fn create_proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        Self::create().proxy(target, aspect)
    }

    /// Creates a default proxy with a default-constructed aspect.
    pub fn create_proxy_default<T, A, R, E, I>(target: T) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + Default + 'static,
    {
        Self::create().proxy_default::<T, A, R, E, I>(target)
    }
}

/// Factory for JDK-style proxies.
#[derive(Debug, Default, Clone, Copy)]
pub struct JdkProxyFactory;

impl JdkProxyFactory {
    /// Creates a JDK-style proxy.
    pub fn proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        ProxyFactory::with_backend(ProxyBackend::Jdk).proxy(target, aspect)
    }
}

/// Factory for CGLIB-style proxies.
#[derive(Debug, Default, Clone, Copy)]
pub struct CglibProxyFactory;

impl CglibProxyFactory {
    /// Creates a CGLIB-style proxy.
    pub fn proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        ProxyFactory::with_backend(ProxyBackend::Cglib).proxy(target, aspect)
    }
}

/// Factory for Spring-CGLIB-style proxies.
#[derive(Debug, Default, Clone, Copy)]
pub struct SpringCglibProxyFactory;

impl SpringCglibProxyFactory {
    /// Creates a Spring-CGLIB-style proxy.
    pub fn proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        ProxyFactory::with_backend(ProxyBackend::SpringCglib).proxy(target, aspect)
    }
}

/// Hutool-aligned convenience facade.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProxyUtil;

impl ProxyUtil {
    /// Creates a default JDK-style aspect proxy.
    pub fn proxy<T, A, R, E, I>(target: T, aspect: I) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        ProxyFactory::create_proxy(target, aspect)
    }

    /// Creates a proxy with a default-constructed aspect type.
    pub fn proxy_default<T, A, R, E, I>(target: T) -> Proxy<T, A, R, E>
    where
        I: Aspect<T, A, R, E> + Default + 'static,
    {
        ProxyFactory::create_proxy_default::<T, A, R, E, I>(target)
    }

    /// Creates an explicit typed invocation-handler proxy.
    pub fn new_proxy_instance<T, H>(target: T, handler: H) -> HandlerProxy<T, H> {
        HandlerProxy::new(target, handler)
    }

    /// Creates a handler proxy while accepting an explicit loader marker.
    ///
    /// Rust does not expose JVM class loaders; the marker lets adapters retain
    /// ownership/lifetime context without affecting dispatch.
    pub fn new_proxy_instance_with_loader<T, H, L>(
        target: T,
        handler: H,
        _loader: &L,
    ) -> HandlerProxy<T, H> {
        HandlerProxy::new(target, handler)
    }
}

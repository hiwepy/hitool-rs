//! 数据源工厂 —— 对齐 Hutool DSFactory 及各连接池工厂名。

use super::{DbSetting, SimpleDataSource};

/// 对齐 Hutool `DSFactory` / `AbstractDSFactory`：按名称创建数据源配置（无全局 current）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DsFactory {
    name: String,
    setting: DbSetting,
}

impl DsFactory {
    /// 对齐 Java: `DSFactory(String dataSourceName)`。
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            setting: DbSetting::new(),
        }
    }

    /// 对齐 Java: `create(Setting)` 的本地等价。
    #[must_use]
    pub fn create(setting: DbSetting) -> Self {
        Self {
            name: "hutool".into(),
            setting,
        }
    }

    /// 对齐 Java: `getSetting()`。
    #[must_use]
    pub fn setting(&self) -> &DbSetting {
        &self.setting
    }

    /// 对齐 Java: `getDataSource()` / `getDataSource(String)`。
    #[must_use]
    pub fn get_data_source(&self, group: &str) -> SimpleDataSource {
        SimpleDataSource::from_config(self.setting.get_db_config(group))
    }

    /// 工厂名称。
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 对齐 Java: `close` / `destroy` —— 清空配置。
    pub fn destroy(&mut self) {
        self.setting = DbSetting::new();
    }
}

/// `AbstractDSFactory` 与 `DSFactory` 同构（显式注入）。
pub type AbstractDsFactory = DsFactory;

/// 各第三方连接池工厂名 → 统一创建 `SimpleDataSource`（不引入 JDBC SPI）。
macro_rules! named_ds_factory {
    ($(#[$meta:meta])* $ty:ident, $label:expr) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $ty {
            inner: DsFactory,
        }

        impl $ty {
            /// 对齐 Java 无参 / Setting 构造的本地等价。
            #[must_use]
            pub fn new() -> Self {
                Self {
                    inner: DsFactory::new($label),
                }
            }

            /// 从 `DbSetting` 构造。
            #[must_use]
            pub fn from_setting(setting: DbSetting) -> Self {
                Self {
                    inner: DsFactory::create(setting),
                }
            }

            /// 对齐 Java: `getDataSource`。
            #[must_use]
            pub fn get_data_source(&self, group: &str) -> SimpleDataSource {
                self.inner.get_data_source(group)
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

named_ds_factory!(
    /// 对齐 Hutool `SimpleDSFactory`。
    SimpleDsFactory,
    "simple"
);
named_ds_factory!(
    /// 对齐 Hutool `PooledDSFactory`。
    PooledDsFactory,
    "pooled"
);
named_ds_factory!(
    /// 对齐 Hutool `HikariDSFactory` —— 仅配置面，不绑定 HikariCP。
    HikariDsFactory,
    "hikari"
);
named_ds_factory!(
    /// 对齐 Hutool `DruidDSFactory`。
    DruidDsFactory,
    "druid"
);
named_ds_factory!(
    /// 对齐 Hutool `DbcpDSFactory`。
    DbcpDsFactory,
    "dbcp"
);
named_ds_factory!(
    /// 对齐 Hutool `C3p0DSFactory`。
    C3p0DsFactory,
    "c3p0"
);
named_ds_factory!(
    /// 对齐 Hutool `TomcatDSFactory`。
    TomcatDsFactory,
    "tomcat"
);
named_ds_factory!(
    /// 对齐 Hutool `BeeDSFactory`。
    BeeDsFactory,
    "bee"
);

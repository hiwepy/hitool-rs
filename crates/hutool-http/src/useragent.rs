//! Browser, rendering-engine, operating-system, and platform detection.

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

/// Name used when a User-Agent component cannot be identified.
pub const UNKNOWN_NAME: &str = "Unknown";

/// Errors produced while registering a custom User-Agent rule.
pub type RuleError = regex::Error;

/// Named User-Agent matching rule.
#[derive(Debug, Clone)]
pub struct UserAgentInfo {
    name: String,
    pattern: Option<Regex>,
}

impl UserAgentInfo {
    /// Creates a case-insensitive matching rule.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        Ok(Self {
            name: name.into(),
            pattern: regex.map(case_insensitive_regex).transpose()?,
        })
    }

    /// Creates an information object from an already compiled pattern.
    #[must_use]
    pub fn from_pattern(name: impl Into<String>, pattern: Option<Regex>) -> Self {
        Self {
            name: name.into(),
            pattern,
        }
    }

    /// Returns the display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the compiled matching pattern, when one exists.
    #[must_use]
    pub const fn pattern(&self) -> Option<&Regex> {
        self.pattern.as_ref()
    }

    /// Returns whether this rule occurs in `content`.
    #[must_use]
    pub fn is_match(&self, content: &str) -> bool {
        self.pattern
            .as_ref()
            .is_some_and(|pattern| pattern.is_match(content))
    }

    /// Returns whether this represents an unidentified component.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.name == UNKNOWN_NAME
    }
}

impl PartialEq for UserAgentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for UserAgentInfo {}

impl Hash for UserAgentInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for UserAgentInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

/// Browser identification rule.
#[derive(Debug, Clone)]
pub struct Browser {
    info: UserAgentInfo,
    version_pattern: Option<Regex>,
    mobile: bool,
}

impl Browser {
    /// Creates a browser rule.
    pub fn new(
        name: impl Into<String>,
        regex: Option<&str>,
        version_regex: Option<&str>,
    ) -> Result<Self, RuleError> {
        let name = name.into();
        Ok(Self {
            mobile: is_mobile_browser_name(&name),
            info: UserAgentInfo::new(name, regex)?,
            version_pattern: version_regex.map(case_insensitive_regex).transpose()?,
        })
    }

    /// Registers a browser rule after the built-in Hutool-compatible rules.
    pub fn add_custom_browser(
        name: impl Into<String>,
        regex: &str,
        version_regex: &str,
    ) -> Result<(), RuleError> {
        let browser = Self::new(name, Some(regex), Some(version_regex))?;
        write_rules(custom_browsers()).push(browser);
        Ok(())
    }

    /// Returns the browser name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the browser version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        capture(self.version_pattern.as_ref(), user_agent)
    }

    /// Returns whether the browser itself represents a mobile client.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        self.mobile
    }

    /// Returns whether this browser is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for Browser {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for Browser {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for Browser {}

impl Hash for Browser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}

/// Rendering-engine identification rule.
#[derive(Debug, Clone)]
pub struct Engine {
    info: UserAgentInfo,
    version_name: Option<String>,
}

impl Engine {
    /// Creates an engine rule and its conventional version matcher.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        let name = name.into();
        let version_name = (name != UNKNOWN_NAME).then(|| name.clone());
        Ok(Self {
            info: UserAgentInfo::new(name, regex)?,
            version_name,
        })
    }

    /// Returns the engine name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the engine version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        engine_version(self.version_name.as_deref()?, user_agent)
    }

    /// Returns whether this engine is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for Engine {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for Engine {}

impl Hash for Engine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}

/// Operating-system identification rule.
#[derive(Debug, Clone)]
pub struct OperatingSystem {
    info: UserAgentInfo,
    version_pattern: Option<Regex>,
}

impl OperatingSystem {
    /// Creates an operating-system rule without version extraction.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        Self::with_version(name, regex, None)
    }

    /// Creates an operating-system rule with version extraction.
    pub fn with_version(
        name: impl Into<String>,
        regex: Option<&str>,
        version_regex: Option<&str>,
    ) -> Result<Self, RuleError> {
        Ok(Self {
            info: UserAgentInfo::new(name, regex)?,
            version_pattern: version_regex.map(case_insensitive_regex).transpose()?,
        })
    }

    /// Registers an operating-system rule after the built-in rules.
    pub fn add_custom_os(
        name: impl Into<String>,
        regex: &str,
        version_regex: &str,
    ) -> Result<(), RuleError> {
        let os = Self::with_version(name, Some(regex), Some(version_regex))?;
        write_rules(custom_operating_systems()).push(os);
        Ok(())
    }

    /// Returns the operating-system name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the operating-system version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        capture(self.version_pattern.as_ref(), user_agent).map(|version| version.replace('_', "."))
    }

    /// Returns whether this is macOS.
    #[must_use]
    pub fn is_macos(&self) -> bool {
        matches!(self.name(), "OSX" | "macOS" | "Mac OS X")
    }

    /// Returns whether this operating system is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for OperatingSystem {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for OperatingSystem {}

impl Hash for OperatingSystem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}

/// Device platform classification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Platform {
    info: UserAgentInfo,
    mobile: bool,
}

impl Platform {
    /// Creates a platform rule.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        let name = name.into();
        Ok(Self {
            mobile: is_mobile_platform_name(&name),
            info: UserAgentInfo::new(name, regex)?,
        })
    }

    /// Returns the platform name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Returns whether this is a mobile platform.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        self.mobile
    }

    /// Returns whether this is an iPhone or iPod.
    #[must_use]
    pub fn is_iphone_or_ipod(&self) -> bool {
        matches!(self.name(), "iPhone" | "iPod")
    }

    /// Returns whether this is an iPad.
    #[must_use]
    pub fn is_ipad(&self) -> bool {
        self.name() == "iPad"
    }

    /// Returns whether this belongs to the iOS family.
    #[must_use]
    pub fn is_ios(&self) -> bool {
        self.is_iphone_or_ipod() || self.is_ipad()
    }

    /// Returns whether this belongs to the Android family.
    #[must_use]
    pub fn is_android(&self) -> bool {
        matches!(self.name(), "Android" | "GoogleTV")
    }

    /// Returns whether this is HarmonyOS/OpenHarmony.
    #[must_use]
    pub fn is_harmony(&self) -> bool {
        self.name() == "Harmony"
    }

    /// Returns whether this platform is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

/// Parsed User-Agent information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAgent {
    mobile: bool,
    browser: Browser,
    version: Option<String>,
    platform: Platform,
    os: OperatingSystem,
    os_version: Option<String>,
    engine: Engine,
    engine_version: Option<String>,
}

impl UserAgent {
    /// Returns whether the client is mobile.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        self.mobile
    }

    /// Overrides the mobile classification.
    pub const fn set_mobile(&mut self, mobile: bool) {
        self.mobile = mobile;
    }

    /// Returns the detected browser.
    #[must_use]
    pub const fn browser(&self) -> &Browser {
        &self.browser
    }

    /// Replaces the detected browser.
    pub fn set_browser(&mut self, browser: Browser) {
        self.browser = browser;
    }

    /// Returns the detected platform.
    #[must_use]
    pub const fn platform(&self) -> &Platform {
        &self.platform
    }

    /// Replaces the detected platform.
    pub fn set_platform(&mut self, platform: Platform) {
        self.platform = platform;
    }

    /// Returns the detected operating system.
    #[must_use]
    pub const fn os(&self) -> &OperatingSystem {
        &self.os
    }

    /// Replaces the detected operating system.
    pub fn set_os(&mut self, os: OperatingSystem) {
        self.os = os;
    }

    /// Returns the operating-system version.
    #[must_use]
    pub fn os_version(&self) -> Option<&str> {
        self.os_version.as_deref()
    }

    /// Replaces the operating-system version.
    pub fn set_os_version(&mut self, version: Option<String>) {
        self.os_version = version;
    }

    /// Returns the detected rendering engine.
    #[must_use]
    pub const fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Replaces the detected rendering engine.
    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = engine;
    }

    /// Returns the browser version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Replaces the browser version.
    pub fn set_version(&mut self, version: Option<String>) {
        self.version = version;
    }

    /// Returns the rendering-engine version.
    #[must_use]
    pub fn engine_version(&self) -> Option<&str> {
        self.engine_version.as_deref()
    }

    /// Replaces the rendering-engine version.
    pub fn set_engine_version(&mut self, version: Option<String>) {
        self.engine_version = version;
    }
}

/// Stateless User-Agent parser facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct UserAgentParser;

impl UserAgentParser {
    /// Parses a non-blank User-Agent string.
    #[must_use]
    pub fn parse(user_agent: &str) -> Option<UserAgent> {
        if user_agent.trim().is_empty() {
            return None;
        }

        let browser = find_browser(user_agent);
        let engine = find_engine(user_agent);
        let os = find_operating_system(user_agent);
        let platform = find_platform(user_agent);
        let mobile = (platform.is_mobile() || browser.is_mobile()) && !os.is_macos();
        Some(UserAgent {
            version: browser.version(user_agent),
            engine_version: engine.version(user_agent),
            os_version: os.version(user_agent),
            mobile,
            browser,
            platform,
            os,
            engine,
        })
    }
}

/// Convenience facade matching Hutool's `UserAgentUtil` role.
#[derive(Debug, Clone, Copy, Default)]
pub struct UserAgentUtil;

impl UserAgentUtil {
    /// Parses a User-Agent string.
    #[must_use]
    pub fn parse(user_agent: &str) -> Option<UserAgent> {
        UserAgentParser::parse(user_agent)
    }
}

fn find_browser(user_agent: &str) -> Browser {
    built_in_browsers()
        .iter()
        .chain(read_rules(custom_browsers()).iter())
        .find(|browser| browser.info.is_match(user_agent))
        .cloned()
        .or_else(|| woothee_browser(user_agent))
        .unwrap_or_else(unknown_browser)
}

fn find_engine(user_agent: &str) -> Engine {
    built_in_engines()
        .iter()
        .find(|engine| engine.info.is_match(user_agent))
        .cloned()
        .unwrap_or_else(unknown_engine)
}

fn find_operating_system(user_agent: &str) -> OperatingSystem {
    built_in_operating_systems()
        .iter()
        .chain(read_rules(custom_operating_systems()).iter())
        .find(|os| os.info.is_match(user_agent))
        .cloned()
        .or_else(|| woothee_operating_system(user_agent))
        .unwrap_or_else(unknown_operating_system)
}

fn find_platform(user_agent: &str) -> Platform {
    built_in_platforms()
        .iter()
        .find(|platform| platform.info.is_match(user_agent))
        .cloned()
        .unwrap_or_else(unknown_platform)
}

fn woothee_browser(user_agent: &str) -> Option<Browser> {
    let result = WootheeParser::new().parse(user_agent)?;
    (result.name != UNKNOWN_NAME)
        .then(|| Browser::new(result.name, None, None).expect("Woothee names are valid"))
}

fn woothee_operating_system(user_agent: &str) -> Option<OperatingSystem> {
    let result = WootheeParser::new().parse(user_agent)?;
    (result.os != UNKNOWN_NAME).then(|| {
        OperatingSystem::new(result.os, None).expect("Woothee operating-system names are valid")
    })
}

fn unknown_browser() -> Browser {
    Browser::new(UNKNOWN_NAME, None, None).expect("empty rule is valid")
}

fn unknown_engine() -> Engine {
    Engine::new(UNKNOWN_NAME, None).expect("empty rule is valid")
}

fn unknown_operating_system() -> OperatingSystem {
    OperatingSystem::new(UNKNOWN_NAME, None).expect("empty rule is valid")
}

fn unknown_platform() -> Platform {
    Platform::new(UNKNOWN_NAME, None).expect("empty rule is valid")
}

fn built_in_browsers() -> &'static [Browser] {
    static RULES: OnceLock<Vec<Browser>> = OnceLock::new();
    RULES.get_or_init(|| {
        [
            ("wxwork", "wxwork", r"wxwork/([\d\w.\-]+)"),
            (
                "WindowsWechat",
                "WindowsWechat",
                r"MicroMessenger[/ ]([\d\w.\-]+)",
            ),
            (
                "MicroMessenger",
                "MicroMessenger",
                r"MicroMessenger[/ ]([\d\w.\-]+)",
            ),
            ("miniProgram", "miniProgram", r"miniProgram[/ ]([\d\w.\-]+)"),
            ("QQBrowser", "QQBrowser", r"QQBrowser/([\d\w.\-]+)"),
            ("DingTalk-win", "dingtalk-win", r"DingTalk\(([\d\w.\-]+)\)"),
            ("DingTalk", "DingTalk", r"AliApp\(DingTalk/([\d\w.\-]+)\)"),
            ("Alipay", "AlipayClient", r"AliApp\(AP/([\d\w.\-]+)\)"),
            ("Taobao", "taobao", r"AliApp\(TB/([\d\w.\-]+)\)"),
            ("UCBrowser", "UC?Browser", r"UC?Browser/([\d\w.\-]+)"),
            (
                "MiuiBrowser",
                "MiuiBrowser|mibrowser",
                r"MiuiBrowser/([\d\w.\-]+)",
            ),
            ("Quark", "Quark", r"Quark[/ ]([\d\w.\-]+)"),
            ("Lenovo", "SLBrowser", r"SLBrowser/([\d\w.\-]+)"),
            ("MSEdge", "Edge|Edg", r"(?:edge|Edg|EdgA)/([\d\w.\-]+)"),
            (
                "Chrome",
                r"chrome|(iphone.*crios.*safari)",
                r"(?:Chrome|CriOS)/([\d\w.\-]+)",
            ),
            ("Firefox", "firefox", r"Firefox[/ ]([\d\w.\-]+)"),
            ("IEMobile", "iemobile", r"IEMobile[/ ]([\d\w.\-]+)"),
            ("Android Browser", "android", r"version/([\d\w.\-]+)"),
            ("Safari", "safari", r"version/([\d\w.\-]+)"),
            ("Opera", "opera", r"Opera[/ ]([\d\w.\-]+)"),
            ("Konqueror", "konqueror", r"Konqueror[/ ]([\d\w.\-]+)"),
            ("PS3", "playstation 3", r"([\d\w.\-]+)\)\s*$"),
            ("PSP", "playstation portable", r"([\d\w.\-]+)\)?\s*$"),
            ("Lotus", r"lotus\.notes", r"Lotus-Notes/([\w.]+)"),
            ("Thunderbird", "thunderbird", r"Thunderbird[/ ]([\d\w.\-]+)"),
            ("Netscape", "netscape", r"Netscape[/ ]([\d\w.\-]+)"),
            ("Seamonkey", "seamonkey", r"Seamonkey[/ ]([\d\w.\-]+)"),
            ("Outlook", r"microsoft\.outlook", r"Outlook[/ ]([\d\w.\-]+)"),
            ("Evolution", "evolution", r"Evolution[/ ]([\d\w.\-]+)"),
            ("MSIE", "msie", r"msie ([\d\w.\-]+)"),
            ("MSIE11", "rv:11", r"rv:([\d\w.\-]+)"),
            ("Gabble", "Gabble", r"Gabble[/ ]([\d\w.\-]+)"),
            ("Yammer Desktop", "AdobeAir", r"([\d\w.\-]+)/Yammer"),
            (
                "Yammer Mobile",
                r"Yammer\s+[\d\w.\-]+",
                r"Yammer\s+([\d\w.\-]+)",
            ),
            (
                "Apache HTTP Client",
                r"Apache\-HttpClient",
                r"Apache\-HttpClient/([\d\w.\-]+)",
            ),
            ("BlackBerry", "BlackBerry", r"BlackBerry[\d]+/([\d\w.\-]+)"),
            ("Baidu", "Baidu", r"baiduboxapp/([\d\w.\-]+)"),
        ]
        .into_iter()
        .map(|(name, regex, version)| {
            Browser::new(name, Some(regex), Some(version)).expect("built-in browser rule is valid")
        })
        .collect()
    })
}

fn built_in_engines() -> &'static [Engine] {
    static RULES: OnceLock<Vec<Engine>> = OnceLock::new();
    RULES.get_or_init(|| {
        [
            ("Trident", "trident"),
            ("Webkit", "webkit"),
            ("Chrome", "chrome"),
            ("Opera", "opera"),
            ("Presto", "presto"),
            ("Gecko", "gecko"),
            ("KHTML", "khtml"),
            ("Konqueror", "konqueror"),
            ("MIDP", "MIDP"),
        ]
        .into_iter()
        .map(|(name, regex)| Engine::new(name, Some(regex)).expect("built-in engine rule is valid"))
        .collect()
    })
}

fn built_in_operating_systems() -> &'static [OperatingSystem] {
    static RULES: OnceLock<Vec<OperatingSystem>> = OnceLock::new();
    RULES.get_or_init(|| {
        [
            (
                "Windows 10 or Windows Server 2016",
                r"windows nt 10\.0",
                r"windows nt (10\.0)",
            ),
            (
                "Windows 8.1 or Windows Server 2012R2",
                r"windows nt 6\.3",
                r"windows nt (6\.3)",
            ),
            (
                "Windows 8 or Windows Server 2012",
                r"windows nt 6\.2",
                r"windows nt (6\.2)",
            ),
            ("Windows Vista", r"windows nt 6\.0", r"windows nt (6\.0)"),
            (
                "Windows 7 or Windows Server 2008R2",
                r"windows nt 6\.1",
                r"windows nt (6\.1)",
            ),
            ("Windows 2003", r"windows nt 5\.2", r"windows nt (5\.2)"),
            ("Windows XP", r"windows nt 5\.1", r"windows nt (5\.1)"),
            ("Windows 2000", r"windows nt 5\.0", r"windows nt (5\.0)"),
            (
                "Windows Phone",
                r"windows (ce|phone|mobile)( os)?",
                r"windows (?:ce|phone|mobile) (\d+(?:[._]\d+)*)",
            ),
            ("Windows", "windows", r"windows(?: nt)? ([\d._]+)"),
            ("OSX", r"os x \d+[._]\d+", r"os x (\d+(?:[._]\d+)*)"),
            ("Android", "Android", r"Android (\d+(?:[._]\d+)*)"),
            ("Harmony", "OpenHarmony", r"OpenHarmony (\d+(?:[._]\d+)*)"),
            ("Android", r"XiaoMi|MI\s+", r"\(X(\d+(?:[._]\d+)*)"),
            ("Linux", "linux", r"Linux[/ ]([\d._]+)"),
            ("Wii", "wii", r"wii libnup/(\d+(?:[._]\d+)*)"),
            ("PS3", "playstation 3", r"playstation 3; (\d+(?:[._]\d+)*)"),
            (
                "PSP",
                "playstation portable",
                r"Portable\); (\d+(?:[._]\d+)*)",
            ),
            (
                "iPad",
                r"\(iPad.*os \d+[._]\d+",
                r"\(iPad.*os (\d+(?:[._]\d+)*)",
            ),
            (
                "iPhone",
                r"\(iPhone.*os \d+[._]\d+",
                r"\(iPhone.*os (\d+(?:[._]\d+)*)",
            ),
            (
                "YPod",
                r"iPod touch[\s;]+iPhone.*os \d+[._]\d+",
                r"iPod touch[\s;]+iPhone.*os (\d+(?:[._]\d+)*)",
            ),
            (
                "YPad",
                r"iPad[\s;]+iPhone.*os \d+[._]\d+",
                r"iPad[\s;]+iPhone.*os (\d+(?:[._]\d+)*)",
            ),
            (
                "YPhone",
                r"iPhone[\s;]+iPhone.*os \d+[._]\d+",
                r"iPhone[\s;]+iPhone.*os (\d+(?:[._]\d+)*)",
            ),
            ("Symbian", "symbian(os)?", r"Symbian(?:OS)?[/ ]([\d._]+)"),
            ("Darwin", r"Darwin/[\d\w.\-]+", r"Darwin/([\d\w.\-]+)"),
            (
                "Adobe Air",
                r"AdobeAir/[\d\w.\-]+",
                r"AdobeAir/([\d\w.\-]+)",
            ),
            ("Java", r"Java\s+[\d\w.\-]+", r"Java\s+([\d\w.\-]+)"),
        ]
        .into_iter()
        .map(|(name, regex, version)| {
            OperatingSystem::with_version(name, Some(regex), Some(version))
                .expect("built-in operating-system rule is valid")
        })
        .collect()
    })
}

fn built_in_platforms() -> &'static [Platform] {
    static RULES: OnceLock<Vec<Platform>> = OnceLock::new();
    RULES.get_or_init(|| {
        [
            ("Windows Phone", r"windows (ce|phone|mobile)( os)?"),
            ("iPad", "ipad"),
            ("iPod", "ipod"),
            ("iPhone", "iphone"),
            ("Android", r"XiaoMi|MI\s+"),
            ("Android", "android"),
            ("GoogleTV", "googletv"),
            ("htcFlyer", "htc_flyer"),
            ("Symbian", "symbian(os)?"),
            ("Blackberry", "blackberry"),
            ("Harmony", "OpenHarmony"),
            ("Windows", "windows"),
            ("Mac", "(macintosh|darwin)"),
            ("Linux", "linux"),
            ("Wii", "wii"),
            ("Playstation", "playstation"),
            ("Java", "java"),
        ]
        .into_iter()
        .map(|(name, regex)| {
            Platform::new(name, Some(regex)).expect("built-in platform rule is valid")
        })
        .collect()
    })
}

fn custom_browsers() -> &'static RwLock<Vec<Browser>> {
    static RULES: OnceLock<RwLock<Vec<Browser>>> = OnceLock::new();
    RULES.get_or_init(|| RwLock::new(Vec::new()))
}

fn custom_operating_systems() -> &'static RwLock<Vec<OperatingSystem>> {
    static RULES: OnceLock<RwLock<Vec<OperatingSystem>>> = OnceLock::new();
    RULES.get_or_init(|| RwLock::new(Vec::new()))
}

fn read_rules<T>(rules: &'static RwLock<Vec<T>>) -> std::sync::RwLockReadGuard<'static, Vec<T>> {
    rules
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn write_rules<T>(rules: &'static RwLock<Vec<T>>) -> std::sync::RwLockWriteGuard<'static, Vec<T>> {
    rules
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn capture(pattern: Option<&Regex>, content: &str) -> Option<String> {
    pattern?
        .captures(content)?
        .get(1)
        .map(|value| value.as_str().to_owned())
}

fn engine_version(name: &str, user_agent: &str) -> Option<String> {
    let start = user_agent
        .to_ascii_lowercase()
        .find(&name.to_ascii_lowercase())?
        + name.len();
    let value = user_agent[start..].strip_prefix(['/', '-', ' '])?;
    let end = value
        .find(|character: char| {
            !character.is_ascii_alphanumeric() && !matches!(character, '.' | '-')
        })
        .unwrap_or(value.len());
    (end > 0).then(|| value[..end].to_owned())
}

fn case_insensitive_regex(pattern: &str) -> Result<Regex, RuleError> {
    RegexBuilder::new(pattern).case_insensitive(true).build()
}

fn is_mobile_browser_name(name: &str) -> bool {
    matches!(
        name,
        "PSP"
            | "Yammer Mobile"
            | "Android Browser"
            | "IEMobile"
            | "MicroMessenger"
            | "miniProgram"
            | "DingTalk"
    )
}

fn is_mobile_platform_name(name: &str) -> bool {
    matches!(
        name,
        "Windows Phone"
            | "iPad"
            | "iPod"
            | "iPhone"
            | "Android"
            | "GoogleTV"
            | "htcFlyer"
            | "Symbian"
            | "Blackberry"
            | "Harmony"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    const CHROME_WINDOWS: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
    const IPHONE_WECHAT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_4 like Mac OS X) AppleWebKit/605.1.15 Mobile/15E148 MicroMessenger/8.0.49";

    fn hash_of(value: &impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn information_rules_compare_hash_and_display_by_name() {
        let first = UserAgentInfo::new("Chrome", Some("chrome")).unwrap();
        let second = UserAgentInfo::from_pattern("Chrome", None);
        assert_eq!(first, second);
        assert_eq!(first.to_string(), "Chrome");
        assert_eq!(first.name(), "Chrome");
        assert!(first.pattern().is_some());
        assert!(first.is_match("CHROME/1"));
        assert!(!second.is_match("Chrome/1"));
        assert!(!first.is_unknown());
        assert!(UserAgentInfo::new(UNKNOWN_NAME, None).unwrap().is_unknown());
        let mut left = DefaultHasher::new();
        let mut right = DefaultHasher::new();
        first.hash(&mut left);
        second.hash(&mut right);
        assert_eq!(left.finish(), right.finish());
        assert!(UserAgentInfo::new("bad", Some("[")).is_err());
    }

    #[test]
    fn named_component_identity_ignores_matching_patterns() {
        let browser_a = Browser::new("Same", Some("first"), Some("first/(\\d+)")).unwrap();
        let browser_b = Browser::new("Same", Some("second"), Some("second/(\\d+)")).unwrap();
        let engine_a = Engine::new("Same", Some("first")).unwrap();
        let engine_b = Engine::new("Same", Some("second")).unwrap();
        let os_a = OperatingSystem::new("Same", Some("first")).unwrap();
        let os_b = OperatingSystem::new("Same", Some("second")).unwrap();
        assert_eq!(browser_a, browser_b);
        assert_eq!(engine_a, engine_b);
        assert_eq!(os_a, os_b);

        assert_eq!(hash_of(&browser_a), hash_of(&browser_b));
        assert_eq!(hash_of(&engine_a), hash_of(&engine_b));
        assert_eq!(hash_of(&os_a), hash_of(&os_b));
        assert_eq!(browser_a.version("unmatched"), None);
        assert_eq!(engine_a.version("unmatched"), None);
        assert_eq!(engine_a.version("Same-1.2"), Some("1.2".to_owned()));
        assert_eq!(engine_a.version("Same 2.3"), Some("2.3".to_owned()));
        assert_eq!(engine_a.version("Same2.3"), None);
        let no_capture = Browser::new("NoCapture", None, Some("matched")).unwrap();
        assert_eq!(no_capture.version("matched"), None);
    }

    #[test]
    fn parses_desktop_chrome_with_versions() {
        let parsed = UserAgentUtil::parse(CHROME_WINDOWS).unwrap();
        assert_eq!(parsed.browser().name(), "Chrome");
        assert_eq!(parsed.version(), Some("124.0.0.0"));
        assert_eq!(parsed.engine().name(), "Webkit");
        assert_eq!(parsed.engine_version(), Some("537.36"));
        assert_eq!(parsed.os().name(), "Windows 10 or Windows Server 2016");
        assert_eq!(parsed.os_version(), Some("10.0"));
        assert_eq!(parsed.platform().name(), "Windows");
        assert!(!parsed.is_mobile());
    }

    #[test]
    fn parses_mobile_wechat_and_ios_helpers() {
        let mut parsed = UserAgentParser::parse(IPHONE_WECHAT).unwrap();
        assert_eq!(parsed.browser().name(), "MicroMessenger");
        assert!(parsed.browser().is_mobile());
        assert_eq!(parsed.version(), Some("8.0.49"));
        assert_eq!(parsed.os().name(), "iPhone");
        assert_eq!(parsed.os_version(), Some("17.4"));
        assert!(parsed.platform().is_iphone_or_ipod());
        assert!(!parsed.platform().is_ipad());
        assert!(parsed.platform().is_ios());
        assert!(!parsed.platform().is_android());
        assert!(!parsed.platform().is_harmony());
        assert!(parsed.is_mobile());

        parsed.set_mobile(false);
        parsed.set_version(Some("custom".into()));
        parsed.set_engine_version(None);
        parsed.set_os_version(None);
        assert!(!parsed.is_mobile());
        assert_eq!(parsed.version(), Some("custom"));
        assert_eq!(parsed.engine_version(), None);
        assert_eq!(parsed.os_version(), None);
    }

    #[test]
    fn mac_desktop_wechat_is_not_mobile() {
        let parsed = UserAgentParser::parse(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_5) AppleWebKit/605.1.15 MicroMessenger/8.0.0 Safari/605.1.15",
        )
        .unwrap();
        assert!(parsed.os().is_macos());
        assert!(!parsed.is_mobile());
    }

    #[test]
    fn platform_family_helpers_cover_all_families() {
        let ipad = Platform::new("iPad", Some("ipad")).unwrap();
        let android = Platform::new("Android", Some("android")).unwrap();
        let television = Platform::new("GoogleTV", Some("googletv")).unwrap();
        let harmony = Platform::new("Harmony", Some("OpenHarmony")).unwrap();
        let unknown = unknown_platform();
        assert!(ipad.is_ipad() && ipad.is_ios() && ipad.is_mobile());
        assert!(android.is_android() && television.is_android());
        assert!(harmony.is_harmony() && harmony.is_mobile());
        assert!(unknown.is_unknown());
        assert_eq!(android.to_string(), "Android");
    }

    #[test]
    fn custom_rules_and_mutators_are_supported() {
        Browser::add_custom_browser("AcmeBrowser", "AcmeBrowser", r"AcmeBrowser/(\d+)").unwrap();
        OperatingSystem::add_custom_os("AcmeOS", "AcmeOS", r"AcmeOS/(\d+)").unwrap();
        let mut parsed = UserAgentParser::parse("AcmeBrowser/7 AcmeOS/9").unwrap();
        assert_eq!(parsed.browser().name(), "AcmeBrowser");
        assert_eq!(parsed.version(), Some("7"));
        assert_eq!(parsed.os().name(), "AcmeOS");
        assert_eq!(parsed.os_version(), Some("9"));

        let browser = Browser::new("PSP", Some("psp"), Some(r"psp/(\d+)")).unwrap();
        let engine = Engine::new("Gecko", Some("gecko")).unwrap();
        let os = OperatingSystem::new("Linux", Some("linux")).unwrap();
        let platform = Platform::new("Linux", Some("linux")).unwrap();
        assert!(browser.is_mobile());
        assert_eq!(browser.to_string(), "PSP");
        assert_eq!(engine.to_string(), "Gecko");
        assert_eq!(os.to_string(), "Linux");
        parsed.set_browser(browser);
        parsed.set_engine(engine);
        parsed.set_os(os);
        parsed.set_platform(platform);
        assert_eq!(parsed.browser().name(), "PSP");
        assert_eq!(parsed.engine().name(), "Gecko");
        assert_eq!(parsed.os().name(), "Linux");
        assert_eq!(parsed.platform().name(), "Linux");
        assert!(Browser::add_custom_browser("bad", "[", "x").is_err());
        assert!(OperatingSystem::add_custom_os("bad", "[", "x").is_err());
    }

    #[test]
    fn blank_unknown_and_woothee_fallbacks_are_safe() {
        assert!(UserAgentParser::parse("   ").is_none());
        let unknown = UserAgentParser::parse("totally-unrecognized-agent").unwrap();
        assert!(unknown.browser().is_unknown());
        assert!(unknown.engine().is_unknown());
        assert!(unknown.os().is_unknown());
        assert!(unknown.platform().is_unknown());
        assert_eq!(unknown.version(), None);
        assert_eq!(unknown.engine_version(), None);
        assert_eq!(unknown.os_version(), None);

        let crawler =
            UserAgentParser::parse("Googlebot/2.1 (+http://www.google.com/bot.html)").unwrap();
        assert!(!crawler.browser().name().is_empty());
    }

    #[test]
    fn constructors_validate_patterns_and_versions() {
        assert!(Browser::new("bad", Some("["), None).is_err());
        assert!(Browser::new("bad", None, Some("[")).is_err());
        assert!(Engine::new("bad[", Some("engine")).is_ok());
        assert!(Engine::new("engine", Some("[")).is_err());
        assert!(OperatingSystem::with_version("bad", None, Some("[")).is_err());
        assert!(Platform::new("bad", Some("[")).is_err());

        let unknown_browser = unknown_browser();
        let unknown_engine = unknown_engine();
        let unknown_os = unknown_operating_system();
        assert!(unknown_browser.is_unknown());
        assert!(unknown_engine.is_unknown());
        assert!(unknown_os.is_unknown());
        assert_eq!(unknown_browser.version("anything"), None);
        assert_eq!(unknown_engine.version("anything"), None);
        assert_eq!(unknown_os.version("anything"), None);
    }

    #[test]
    fn hutool_legacy_browser_catalog_remains_recognizable() {
        let cases = [
            ("PlayStation 3 foo 4.90)", "PS3"),
            ("PlayStation Portable foo 6.61)", "PSP"),
            ("Lotus.Notes Lotus-Notes/9.0", "Lotus"),
            ("Netscape/9.0", "Netscape"),
            ("Seamonkey/2.53", "Seamonkey"),
            ("Microsoft.Outlook Outlook/16.0", "Outlook"),
            ("Evolution/3.50", "Evolution"),
            ("Gabble/1.2", "Gabble"),
            ("AdobeAir 1.0/Yammer", "Yammer Desktop"),
            ("Yammer 7.8", "Yammer Mobile"),
            ("Apache-HttpClient/5.4", "Apache HTTP Client"),
            ("BlackBerry9700/7.1", "BlackBerry"),
        ];
        for (source, expected) in cases {
            assert_eq!(
                UserAgentParser::parse(source).unwrap().browser().name(),
                expected
            );
        }
    }

    #[test]
    fn hutool_legacy_os_and_platform_catalog_remains_recognizable() {
        let os_cases = [
            ("Windows NT 6.0", "Windows Vista"),
            ("Windows NT 5.2", "Windows 2003"),
            ("Windows NT 5.0", "Windows 2000"),
            ("Windows Phone 10.0", "Windows Phone"),
            ("Wii libnup/5.5", "Wii"),
            ("PlayStation 3; 4.90", "PS3"),
            ("PlayStation Portable); 6.61", "PSP"),
            ("SymbianOS/9.4", "Symbian"),
            ("AdobeAir/33.1", "Adobe Air"),
        ];
        for (source, expected) in os_cases {
            assert_eq!(
                UserAgentParser::parse(source).unwrap().os().name(),
                expected
            );
        }
        assert_eq!(
            UserAgentParser::parse("htc_flyer")
                .unwrap()
                .platform()
                .name(),
            "htcFlyer"
        );
        assert_eq!(
            UserAgentParser::parse("Wii").unwrap().platform().name(),
            "Wii"
        );
    }
}

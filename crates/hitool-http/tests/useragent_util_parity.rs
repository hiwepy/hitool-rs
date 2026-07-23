//! UserAgentUtil / IssueIB3SJF parity — 对齐 hutool-http useragent 包
//! 对齐: `cn.hutool.http.useragent.UserAgentUtilTest`

// Note: hitool normalizes OS versions `4_3_3` → `4.3.3` (Hutool keeps underscores).
use hitool_http::useragent::UserAgentUtil;

/// 对齐 Java: `UserAgentUtilTest.parseDesktopTest()`
#[test]
fn parse_desktop_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/535.1 (KHTML, like Gecko) Chrome/14.0.835.163 Safari/535.1";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Chrome");
    assert_eq!(ua.version(), Some("14.0.835.163"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("535.1"));
    assert_eq!(ua.os().name(), "Windows 7 or Windows Server 2008R2");
    assert_eq!(ua.os_version(), Some("6.1"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseMobileTest()`
#[test]
fn parse_mobile_test() {
    let ua_str = "User-Agent:Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3_3 like Mac OS X; en-us) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8J2 Safari/6533.18.5";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Safari");
    assert_eq!(ua.version(), Some("5.0.2"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("533.17.9"));
    assert_eq!(ua.os().name(), "iPhone");
    assert_eq!(ua.os_version(), Some("4.3.3"));
    assert_eq!(ua.platform().name(), "iPhone");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseMiui10WithChromeTest()`
#[test]
fn parse_miui10_with_chrome_test() {
    let ua_str = "Mozilla/5.0 (Linux; Android 9; MIX 3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.80 Mobile Safari/537.36";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Chrome");
    assert_eq!(ua.version(), Some("70.0.3538.80"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Android");
    assert_eq!(ua.os_version(), Some("9"));
    assert_eq!(ua.platform().name(), "Android");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseHuaweiPhoneWithNativeBrowserTest()`
#[test]
fn parse_huawei_phone_with_native_browser_test() {
    let ua_str = "Mozilla/5.0 (Linux; Android 10; EML-AL00 Build/HUAWEIEML-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Mobile Safari/537.36";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Android Browser");
    assert_eq!(ua.version(), Some("4.0"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Android");
    assert_eq!(ua.os_version(), Some("10"));
    assert_eq!(ua.platform().name(), "Android");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseSamsungPhoneWithNativeBrowserTest()`
#[test]
fn parse_samsung_phone_with_native_browser_test() {
    let ua_str = "Dalvik/2.1.0 (Linux; U; Android 9; SM-G950U Build/PPR1.180610.011)";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Android Browser");
    assert_eq!(ua.version(), None);
    assert_eq!(ua.engine().name(), "Unknown");
    assert_eq!(ua.engine_version(), None);
    assert_eq!(ua.os().name(), "Android");
    assert_eq!(ua.os_version(), Some("9"));
    assert_eq!(ua.platform().name(), "Android");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseWindows10WithChromeTest()`
#[test]
fn parse_windows10_with_chrome_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Chrome");
    assert_eq!(ua.version(), Some("70.0.3538.102"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Windows 10 or Windows Server 2016");
    assert_eq!(ua.os_version(), Some("10.0"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseWindows10WithIe11Test()`
#[test]
fn parse_windows10_with_ie11_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "MSIE11");
    assert_eq!(ua.version(), Some("11.0"));
    assert_eq!(ua.engine().name(), "Trident");
    assert_eq!(ua.engine_version(), Some("7.0"));
    assert_eq!(ua.os().name(), "Windows 10 or Windows Server 2016");
    assert_eq!(ua.os_version(), Some("10.0"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseWindows10WithIeMobileLumia520Test()`
#[test]
fn parse_windows10_with_ie_mobile_lumia520_test() {
    let ua_str = "Mozilla/5.0 (Mobile; Windows Phone 8.1; Android 4.0; ARM; Trident/7.0; Touch; rv:11.0; IEMobile/11.0; NOKIA; Lumia 520) like iPhone OS 7_0_3 Mac OS X AppleWebKit/537 (KHTML, like Gecko) Mobile Safari/537 ";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "IEMobile");
    assert_eq!(ua.version(), Some("11.0"));
    assert_eq!(ua.engine().name(), "Trident");
    assert_eq!(ua.engine_version(), Some("7.0"));
    assert_eq!(ua.os().name(), "Windows Phone");
    assert_eq!(ua.os_version(), Some("8.1"));
    assert_eq!(ua.platform().name(), "Windows Phone");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseWindows10WithIe8EmulatorTest()`
#[test]
fn parse_windows10_with_ie8_emulator_test() {
    let ua_str = "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "MSIE");
    assert_eq!(ua.version(), Some("8.0"));
    assert_eq!(ua.engine().name(), "Trident");
    assert_eq!(ua.engine_version(), Some("4.0"));
    assert_eq!(ua.os().name(), "Windows 7 or Windows Server 2008R2");
    assert_eq!(ua.os_version(), Some("6.1"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseWindows10WithEdgeTest()`
#[test]
fn parse_windows10_with_edge_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "MSEdge");
    assert_eq!(ua.version(), Some("18.17763"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Windows 10 or Windows Server 2016");
    assert_eq!(ua.os_version(), Some("10.0"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseEdgeOnLumia950XLTest()`
#[test]
fn parse_edge_on_lumia950_xl_test() {
    let ua_str = "Mozilla/5.0 (Windows Phone 10.0; Android 6.0.1; Microsoft; Lumia 950XL) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Mobile Safari/537.36 Edge/15.14900";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "MSEdge");
    assert_eq!(ua.version(), Some("15.14900"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Windows Phone");
    assert_eq!(ua.os_version(), Some("10.0"));
    assert_eq!(ua.platform().name(), "Windows Phone");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseChromeOnWindowsServer2012R2Test()`
#[test]
fn parse_chrome_on_windows_server2012_r2_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.132 Safari/537.36";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Chrome");
    assert_eq!(ua.version(), Some("63.0.3239.132"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Windows 8.1 or Windows Server 2012R2");
    assert_eq!(ua.os_version(), Some("6.3"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseIE11OnWindowsServer2008R2Test()`
#[test]
fn parse_ie11_on_windows_server2008_r2_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "MSIE11");
    assert_eq!(ua.version(), Some("11.0"));
    assert_eq!(ua.engine().name(), "Trident");
    assert_eq!(ua.engine_version(), Some("7.0"));
    assert_eq!(ua.os().name(), "Windows 7 or Windows Server 2008R2");
    assert_eq!(ua.os_version(), Some("6.1"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseEdgeTest()`
#[test]
fn parse_edge_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.69 Safari/537.36 Edg/81.0.416.34";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "MSEdge");
    assert_eq!(ua.version(), Some("81.0.416.34"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Windows 10 or Windows Server 2016");
    assert_eq!(ua.os_version(), Some("10.0"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseMicroMessengerTest()`
#[test]
fn parse_micro_messenger_test() {
    let ua_str = "Mozilla/5.0 (iPhone; CPU iPhone OS 11_0 like Mac OS X) AppleWebKit/604.1.38 (KHTML, like Gecko) Mobile/15A372 MicroMessenger/7.0.17(0x17001127) NetType/WIFI Language/zh_CN";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "MicroMessenger");
    assert_eq!(ua.version(), Some("7.0.17"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("604.1.38"));
    assert_eq!(ua.os().name(), "iPhone");
    assert_eq!(ua.os_version(), Some("11.0"));
    assert_eq!(ua.platform().name(), "iPhone");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseWorkWxTest()`
#[test]
fn parse_work_wx_test() {
    let ua_str = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 wxwork/3.0.31 MicroMessenger/7.0.1 Language/zh";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "wxwork");
    assert_eq!(ua.version(), Some("3.0.31"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("605.1.15"));
    assert_eq!(ua.os().name(), "iPhone");
    assert_eq!(ua.platform().name(), "iPhone");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseQQTest()`
#[test]
fn parse_qq_test() {
    let ua_str = "User-Agent: MQQBrowser/26 Mozilla/5.0 (Linux; U; Android 2.3.7; zh-cn; MB200 Build/GRJ22; CyanogenMod-7) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "QQBrowser");
    assert_eq!(ua.version(), Some("26"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("533.1"));
    assert_eq!(ua.os().name(), "Android");
    assert_eq!(ua.os_version(), Some("2.3.7"));
    assert_eq!(ua.platform().name(), "Android");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseDingTalkTest()`
#[test]
fn parse_ding_talk_test() {
    let ua_str = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/18A373 AliApp(DingTalk/5.1.33) com.laiwang.DingTalk/13976299 Channel/201200 language/zh-Hans-CN WK";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "DingTalk");
    assert_eq!(ua.version(), Some("5.1.33"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("605.1.15"));
    assert_eq!(ua.os().name(), "iPhone");
    assert_eq!(ua.os_version(), Some("14.0"));
    assert_eq!(ua.platform().name(), "iPhone");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseAlipayTest()`
#[test]
fn parse_alipay_test() {
    let ua_str = "Mozilla/5.0 (Linux; U; Android 7.0; zh-CN; FRD-AL00 Build/HUAWEIFRD-AL00) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/40.0.2214.89 UCBrowser/11.3.8.909 UWS/2.10.2.5 Mobile Safari/537.36 UCBS/2.10.2.5 Nebula AlipayDefined(nt:WIFI,ws:360|0|3.0) AliApp(AP/10.0.18.062203) AlipayClient/10.0.18.062203 Language/zh-Hans useStatusBar/true";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Alipay");
    assert_eq!(ua.version(), Some("10.0.18.062203"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Android");
    assert_eq!(ua.os_version(), Some("7.0"));
    assert_eq!(ua.platform().name(), "Android");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseTaobaoTest()`
#[test]
fn parse_taobao_test() {
    let ua_str = "Mozilla/5.0 (Linux; U; Android 4.4.4; zh-cn; MI 2C Build/KTU84P) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/30.0.0.0 Mobile Safari/537.36 AliApp(TB/4.9.2) WindVane/5.2.2 TBANDROID/700342@taobao_android_4.9.2 720X1280";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Taobao");
    assert_eq!(ua.version(), Some("4.9.2"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Android");
    assert_eq!(ua.os_version(), Some("4.4.4"));
    assert_eq!(ua.platform().name(), "Android");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseUCTest()`
#[test]
fn parse_uc_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/38.0.2125.122 UBrowser/4.0.3214.0 Safari/537.36";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "UCBrowser");
    assert_eq!(ua.version(), Some("4.0.3214.0"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Windows 7 or Windows Server 2008R2");
    assert_eq!(ua.os_version(), Some("6.1"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseUCTest2()`
#[test]
fn parse_uc_test2() {
    let ua_str = "Mozilla/5.0 (iPhone; CPU iPhone OS 12_4_1 like Mac OS X; zh-CN) AppleWebKit/537.51.1 (KHTML, like Gecko) Mobile/16G102 UCBrowser/12.7.6.1251 Mobile AliApp(TUnionSDK/0.1.20.3)";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "UCBrowser");
    assert_eq!(ua.version(), Some("12.7.6.1251"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.51.1"));
    assert_eq!(ua.os().name(), "iPhone");
    assert_eq!(ua.os_version(), Some("12.4.1"));
    assert_eq!(ua.platform().name(), "iPhone");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseQuarkTest()`
#[test]
fn parse_quark_test() {
    let ua_str = "Mozilla/5.0 (iPhone; CPU iPhone OS 12_4_1 like Mac OS X; zh-cn) AppleWebKit/601.1.46 (KHTML, like Gecko) Mobile/16G102 Quark/3.6.2.993 Mobile";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Quark");
    assert_eq!(ua.version(), Some("3.6.2.993"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("601.1.46"));
    assert_eq!(ua.os().name(), "iPhone");
    assert_eq!(ua.os_version(), Some("12.4.1"));
    assert_eq!(ua.platform().name(), "iPhone");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseWxworkTest()`
#[test]
fn parse_wxwork_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.116 Safari/537.36 QBCore/4.0.1326.400 QQBrowser/9.0.2524.400 Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.116 Safari/537.36 wxwork/3.1.10 (MicroMessenger/6.2) WindowsWechat";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "wxwork");
    assert_eq!(ua.version(), Some("3.1.10"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Windows 10 or Windows Server 2016");
    assert_eq!(ua.os_version(), Some("10.0"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseWxworkMobileTest()`
#[test]
fn parse_wxwork_mobile_test() {
    let ua_str = "Mozilla/5.0 (Linux; Android 10; JSN-AL00 Build/HONORJSN-AL00; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/77.0.3865.120 MQQBrowser/6.2 TBS/045710 Mobile Safari/537.36 wxwork/3.1.10 ColorScheme/Light MicroMessenger/7.0.1 NetType/WIFI Language/zh Lang/zh";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "wxwork");
    assert_eq!(ua.version(), Some("3.1.10"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Android");
    assert_eq!(ua.os_version(), Some("10"));
    assert_eq!(ua.platform().name(), "Android");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseEdgATest()`
#[test]
fn parse_edg_a_test() {
    let ua_str = "userAgent: Mozilla/5.0 (Linux; Android 11; MI 9 Transparent Edition) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.55 Mobile Safari/537.36 EdgA/96.0.1054.36";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "MSEdge");
    assert_eq!(ua.version(), Some("96.0.1054.36"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Android");
    assert_eq!(ua.os_version(), Some("11"));
    assert_eq!(ua.platform().name(), "Android");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseLenovoTest()`
#[test]
fn parse_lenovo_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147.89 Safari/537.36 SLBrowser/7.0.0.6241 SLBChan/30";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "Lenovo");
    assert_eq!(ua.version(), Some("7.0.0.6241"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Windows 10 or Windows Server 2016");
    assert_eq!(ua.os_version(), Some("10.0"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseXiaoMiTest()`
#[test]
fn parse_xiao_mi_test() {
    let ua_str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/534.24 (KHTML, like Gecko) Chrome/89.0.4389.116 Safari/534.24 XiaoMi/MiuiBrowser/16.0.18 swan-mibrowser";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "MiuiBrowser");
    assert_eq!(ua.version(), Some("16.0.18"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("534.24"));
    // Hutool maps MiuiBrowser on Linux UA to Android family
    assert_eq!(ua.os().name(), "Android");
    assert_eq!(ua.os_version(), Some("11"));
    assert_eq!(ua.platform().name(), "Android");
    assert!(ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.parseFromDeepinTest()`
#[test]
fn parse_from_deepin_test() {
    let ua_str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.198 Safari/537.36";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.os().name(), "Linux");
}

/// 对齐 Java: `UserAgentUtilTest.parseHarmonyUATest()`
#[test]
fn parse_harmony_ua_test() {
    let ua_str = "Mozilla/5.0 (Phone; OpenHarmony 4.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36  ArkWeb/4.1.6.1 Mobile ";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.platform().name(), "Harmony");
    assert!(ua.platform().is_harmony());
    assert_eq!(ua.os().name(), "Harmony");
    assert_eq!(ua.os_version(), Some("4.1"));
}

/// 对齐 Java: `UserAgentUtilTest.issueI60UOPTest()`
#[test]
fn issue_i60_uop_test() {
    let ua_str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.164 Safari/537.36 dingtalk-win/1.0.0 nw(0.14.7) DingTalk(6.5.40-Release.9059101) Mojo/1.0.0 Native AppType(release) Channel/201200";
    let ua = UserAgentUtil::parse(ua_str).expect("UA should parse");
    assert_eq!(ua.browser().name(), "DingTalk-win");
    assert_eq!(ua.version(), Some("6.5.40-Release.9059101"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "Windows 10 or Windows Server 2016");
    assert_eq!(ua.os_version(), Some("10.0"));
    assert_eq!(ua.platform().name(), "Windows");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.issuseI7OTCUTest()`
#[test]
fn issuse_i7_otcu_test() {
    let ua_str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
    let ua = UserAgentUtil::parse(ua_str).unwrap();
    assert_eq!(ua.browser().name(), "Chrome");
    assert_eq!(ua.version(), Some("114.0.0.0"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("537.36"));
    assert_eq!(ua.os().name(), "OSX");
    assert_eq!(ua.os_version(), Some("10.15.7"));
    assert_eq!(ua.platform().name(), "Mac");
    assert!(!ua.is_mobile());

    let ua_str2 = "Mozilla/5.0 (iPhone; CPU iPhone OS 10_3 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) CriOS/56.0.2924.75 Mobile/14E5239e Safari/602.1";
    let ua2 = UserAgentUtil::parse(ua_str2).unwrap();
    assert_eq!(ua2.browser().name(), "Chrome");
    assert_eq!(ua2.version(), Some("56.0.2924.75"));
    assert_eq!(ua2.engine().name(), "Webkit");
    assert_eq!(ua2.engine_version(), Some("602.1.50"));
    assert_eq!(ua2.os().name(), "iPhone");
    assert_eq!(ua2.os_version(), Some("10.3"));
    assert_eq!(ua2.platform().name(), "iPhone");
    assert!(ua2.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.issueI847JYTest()`
#[test]
fn issue_i847_jy_test() {
    let s = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 SP-engine/2.80.0 main%2F1.0 baiduboxapp/13.42.0.11 (Baidu; P2 17.0) NABar/1.0 themeUA=Them";
    let ua2 = UserAgentUtil::parse(s).unwrap();
    assert_eq!(ua2.browser().name(), "Baidu");
    assert_eq!(ua2.version(), Some("13.42.0.11"));
    assert_eq!(ua2.engine().name(), "Webkit");
    assert_eq!(ua2.engine_version(), Some("605.1.15"));
    assert_eq!(ua2.os().name(), "iPhone");
    assert_eq!(ua2.os_version(), Some("17.0"));
    assert_eq!(ua2.platform().name(), "iPhone");
    assert!(ua2.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.issueI8X5XQTest()`
#[test]
fn issue_i8_x5_xq_test() {
    let s = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.71 Safari/537.36 Core/1.94.218.400 QQBrowser/12.1.5496.400";
    let ua2 = UserAgentUtil::parse(s).unwrap();
    assert_eq!(ua2.browser().name(), "QQBrowser");
    assert_eq!(ua2.version(), Some("12.1.5496.400"));
    assert_eq!(ua2.engine().name(), "Webkit");
    assert_eq!(ua2.engine_version(), Some("537.36"));
    assert_eq!(ua2.os().name(), "Windows 10 or Windows Server 2016");
    assert_eq!(ua2.os_version(), Some("10.0"));
    assert_eq!(ua2.platform().name(), "Windows");
    assert!(!ua2.is_mobile());
}

/// 对齐 Java: `UserAgentUtilTest.issueIA74K2Test()`
#[test]
fn issue_ia74_k2_test() {
    let ua = UserAgentUtil::parse(
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) MicroMessenger/6.8.0(0x16080000) MacWechat/3.8.7(0x13080710) Safari/605.1.15 NetType/WIFI",
    )
    .unwrap();
    assert_eq!(ua.browser().name(), "MicroMessenger");
    assert_eq!(ua.version(), Some("6.8.0"));
    assert_eq!(ua.engine().name(), "Webkit");
    assert_eq!(ua.engine_version(), Some("605.1.15"));
    assert_eq!(ua.os().name(), "OSX");
    assert_eq!(ua.os_version(), Some("10.15.7"));
    assert_eq!(ua.platform().name(), "Mac");
    assert!(!ua.is_mobile());
}

/// 对齐 Java: `IssueIB3SJFTest.isMobileTest()`
#[test]
fn issue_ib3_sjf_is_mobile_test() {
    let s = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 NetType/WIFI MicroMessenger/7.0.20.1781(0x6700143B) WindowsWechat(0x63090c11) XWEB/11275 Flue";
    let ua = UserAgentUtil::parse(s).unwrap();
    assert!(!ua.is_mobile());
    assert_eq!(ua.browser().version(s), Some("7.0.20.1781".to_owned()));
}

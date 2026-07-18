use hitool_captcha as hc;
use hc::CodeGenerator;

#[test]
fn alphanumeric_generator_test() {
    let generator = hc::AlphanumericGenerator::new(8);
    let code = generator.generate();
    assert_eq!(code.len(), 8, "AlphanumericGenerator 长度应为 8");
}

#[test]
fn captcha_challenge_generate_test() {
    let generator = hc::AlphanumericGenerator::new(4);
    let challenge = hc::CaptchaChallenge::generate(
        &generator,
        std::time::Duration::from_secs(300),
    );
    // generate 返回 CaptchaChallenge，不是 Result
    assert!(!challenge.code().is_empty(), "captcha code 不应为空");
}

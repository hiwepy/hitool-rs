use hitool_aop as ha;

#[test]
fn interceptor_chain_test() {
    let chain = ha::InterceptorChain::<i32, String, String>::new();
    assert!(chain.is_empty(), "空 chain 应为空");
}

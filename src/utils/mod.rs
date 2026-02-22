#[macro_export]
macro_rules! printlns_simple {
    // 核心修复1：匹配「0个/多个参数」（含末尾逗号），统一处理为切片
    ($($args:expr),* $(,)?) => {{
        // 核心修复2：直接用 &[&dyn Debug] 动态分发，兼容所有 Debug 类型
        let items: &[&dyn std::fmt::Debug] = &[$(&$args),*];
        // 核心修复3：简化打印逻辑，无需内部函数，避免类型嵌套
        println!("{{{:?}}}", items);
    }};
}

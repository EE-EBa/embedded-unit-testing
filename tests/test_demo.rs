#![no_std]
#![no_main]
use cortex_m_rt;/* 必须添加这个，提供中断向量表 */
use cortex_m;/* 必须添加，提供critical-section */


macro_rules! bit_proxy_v1 {
    ($writer:ident,$marker:ident) => {
        /* 隐藏api文档信息，为内部实现细节 */
        #[doc(hidden)]
        pub struct $marker;

        /* 生成写入器类型别名 */
        pub type $writer = u32;
    };
}

bit_proxy_v1!(MyWriter, MyMarker);


use rtt_target;
#[cfg(test)]
#[embedded_test::tests]
mod test_v1 {
    use super::*;

    #[test]
    #[cfg(feature = "log")]
    fn test_basic_generation() {
        rtt_target::rtt_init_log!();
        /* 验证类型是否正确 */
        let _marker = MyMarker;
        let _writer: MyWriter = 42u32;
        log::info!("\n第一步测试通过：基础宏结构创建成功\n");
    }
}

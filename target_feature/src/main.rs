use owo_colors::OwoColorize;

macro_rules! print_x86_feature {
    // use :tt instead of :literal because of transcribing behavior when forwarding fragments:
    // https://doc.rust-lang.org/stable/reference/macros-by-example.html?highlight=forwarding#transcribing
    ($feature:tt) => {
        let _enabled = false;
        #[cfg(target_feature = $feature)]
        let _enabled = true;
        if _enabled {
            println!("{}: {}", $feature, "enabled".green());
        } else {
            if is_x86_feature_detected!($feature) {
                println!("{}: {}", $feature, "disabled but available".bright_yellow());
            } else {
                println!("{}: {}", $feature, "not available".bright_black());
            }
        }
    };
}

fn main() {
    print_x86_feature!("aes");
    print_x86_feature!("pclmulqdq");
    print_x86_feature!("rdrand");
    print_x86_feature!("rdseed");
    print_x86_feature!("tsc");
    print_x86_feature!("mmx");
    print_x86_feature!("sse");
    print_x86_feature!("sse2");
    print_x86_feature!("sse3");
    print_x86_feature!("ssse3");
    print_x86_feature!("sse4.1");
    print_x86_feature!("sse4.2");
    print_x86_feature!("sse4a");
    print_x86_feature!("sha");
    print_x86_feature!("avx");
    print_x86_feature!("avx2");
    print_x86_feature!("avx512f");
    print_x86_feature!("avx512cd");
    print_x86_feature!("avx512er");
    print_x86_feature!("avx512pf");
    print_x86_feature!("avx512bw");
    print_x86_feature!("avx512dq");
    print_x86_feature!("avx512vl");
    print_x86_feature!("avx512ifma");
    print_x86_feature!("avx512vbmi");
    print_x86_feature!("avx512vpopcntdq");
    print_x86_feature!("avx512vbmi2");
    print_x86_feature!("avx512gfni");
    print_x86_feature!("avx512vaes");
    print_x86_feature!("avx512vpclmulqdq");
    print_x86_feature!("avx512vnni");
    print_x86_feature!("avx512bitalg");
    print_x86_feature!("avx512bf16");
    print_x86_feature!("avx512vp2intersect");
    print_x86_feature!("f16c");
    print_x86_feature!("fma");
    print_x86_feature!("bmi1");
    print_x86_feature!("bmi2");
    print_x86_feature!("abm");
    print_x86_feature!("lzcnt");
    print_x86_feature!("tbm");
    print_x86_feature!("popcnt");
    print_x86_feature!("fxsr");
    print_x86_feature!("xsave");
    print_x86_feature!("xsaveopt");
    print_x86_feature!("xsaves");
    print_x86_feature!("xsavec");
    print_x86_feature!("cmpxchg16b");
    print_x86_feature!("adx");
    print_x86_feature!("rtm");
}

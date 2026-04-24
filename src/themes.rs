use std::sync::LazyLock;

use crate::api::types::Argb;

macro_rules! theme {
    ($name:ident, $theme:expr) => {
        pub static $name: LazyLock<Theme> = LazyLock::new(|| $theme);
    };
}

pub fn set_alpha(color: Argb, alpha: f32) -> Argb {
    assert!((0.0..=1.0).contains(&alpha));

    Argb {
        a: (alpha * 255.0) as u8,
        r: color.r,
        g: color.g,
        b: color.b,
    }
}

pub struct Theme {
    pub rosewater: Argb,
    pub flamingo: Argb,
    pub pink: Argb,
    pub mauve: Argb,
    pub red: Argb,
    pub maroon: Argb,
    pub peach: Argb,
    pub yellow: Argb,
    pub green: Argb,
    pub teal: Argb,
    pub sky: Argb,
    pub sapphire: Argb,
    pub blue: Argb,
    pub lavender: Argb,
    pub text: Argb,
    pub subtext1: Argb,
    pub subtext0: Argb,
    pub overlay2: Argb,
    pub overlay1: Argb,
    pub overlay0: Argb,
    pub surface2: Argb,
    pub surface1: Argb,
    pub surface0: Argb,
    pub base: Argb,
    pub mantle: Argb,
    pub crust: Argb,
    pub transparent: Argb,
}

theme!(
    CATPUCCIN_MOCHA,
    Theme {
        rosewater: "0xfff5e0dc".parse().unwrap(),
        flamingo: "0xfff2cdcd".parse().unwrap(),
        pink: "0xfff5c2e7".parse().unwrap(),
        mauve: "0xffcba6f7".parse().unwrap(),
        red: "0xfff38ba8".parse().unwrap(),
        maroon: "0xffeba0ac".parse().unwrap(),
        peach: "0xfffab387".parse().unwrap(),
        yellow: "0xfff9e2af".parse().unwrap(),
        green: "0xffa6e3a1".parse().unwrap(),
        teal: "0xff94e2d5".parse().unwrap(),
        sky: "0xff89dceb".parse().unwrap(),
        sapphire: "0xff74c7ec".parse().unwrap(),
        blue: "0xff8caaee".parse().unwrap(),
        lavender: "0xffb4befe".parse().unwrap(),
        text: "0xffcdd6f4".parse().unwrap(),
        subtext1: "0xffbac2de".parse().unwrap(),
        subtext0: "0xffa6adc8".parse().unwrap(),
        overlay2: "0xff9399b2".parse().unwrap(),
        overlay1: "0xff7f849c".parse().unwrap(),
        overlay0: "0xff6c7086".parse().unwrap(),
        surface2: "0xff585b70".parse().unwrap(),
        surface1: "0xff45475a".parse().unwrap(),
        surface0: "0xff313244".parse().unwrap(),
        base: "0xff1e1e2e".parse().unwrap(),
        mantle: "0xff181825".parse().unwrap(),
        crust: "0xff11111b".parse().unwrap(),
        transparent: "0x00000000".parse().unwrap(),
    }
);

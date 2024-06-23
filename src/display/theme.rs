use std::ops::Deref;

use html::ImplicitClone;
use once_cell::sync::Lazy;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub gradient_dark: String,
    pub gradient_light: String,
    pub paper: String,
    pub paper_secondary: String,
    pub navbar: String,
    pub sidebar: String,
    pub text_default: String,
    pub text_invert: String,
    pub text_colored: String,
    pub h1: String,
    pub h2: String,
    pub h3: String,
    pub h4: String,
    pub h5: String,
    pub h6: String,
    pub hr: String,
    pub link: String,
    pub link_hover: String,
    pub link_visited: String,
    pub panel_color_primary: String,
    pub panel_color_secondary: String,
    pub panel_color_dark: String,
    pub scroll_bar: String,
    pub scroll_bar_hover: String,
    pub banner_cube_color: String,
    pub banner_cube_border_color: String,
    pub banner_sphere_color: String,
    pub banner_color_one: String,
    pub banner_color_two: String,
    pub banner_color_three: String,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ThemeKind {
    Dark,
    Light,
}

impl ImplicitClone for ThemeKind {}

impl ThemeKind {
    pub fn current(&self) -> &Theme {
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            // gradient_dark: "#656d4a".to_string(),
            // gradient_light: "#c2c5aa".to_string(),
            // navbar: "#b6ad90".to_string(),
            // sidebar: "#333d29".to_string(),
            // text_default: "#582f0e".to_string(),
            // text_invert: "#936639".to_string(),
            // gradient_dark: "#1a1c1a".to_string(),
            // gradient_light: "#1d2118".to_string(),
            // navbar: "#414833".to_string(),
            // sidebar: "#333d29".to_string(),
            // text_default: "#936639".to_string(),
            // text_invert: "#582f0e".to_string(),
            // text_colored: "#c2c5aa".to_string(),
            // image_grad_overlay: "".to_string(),
            // banner_cube_color: "rgba(133, 87, 46, 0.001)".to_string(),
            // banner_cube_border_color: "rgba(224, 227, 161, 0.2)".to_string(),
            // banner_sphere_color: "rgba(88, 97, 45, 0.05)".to_string(),
            // banner_color_one: "#656d4a".to_string(),
            // banner_color_two: "#ecf39e".to_string(),
            // banner_color_three: "#936639".to_string(),
            gradient_dark: "#d6b99c".to_string(),
            gradient_light: "#c2c5aa".to_string(),
            paper: "#c2c5aa".to_string(),
            paper_secondary: "#b6ad90".to_string(),
            navbar: "#414833".to_string(),
            sidebar: "#b6ad90".to_string(),
            text_default: "#1d2118".to_string(),
            text_invert: "#c2c5aa".to_string(),
            text_colored: "#7f4f24".to_string(),
            h1: "#582f0e".to_string(),
            h2: "#332403".to_string(),
            h3: "#333d29".to_string(),
            h4: "#3c1642".to_string(),
            h5: "#132a13".to_string(),
            h6: "#081c15".to_string(),
            hr: "#582f0e".to_string(),
            link: "#7f4f24".to_string(),
            link_hover: "#582f0e".to_string(),
            link_visited: "#3c1642".to_string(),
            panel_color_primary: "#656d4a".to_string(),
            panel_color_secondary: "#545c3b".to_string(),
            panel_color_dark: "#333d29".to_string(),
            scroll_bar: "#333d29".to_string(),
            scroll_bar_hover: "#132a13".to_string(),
            banner_cube_color: "rgba(127, 79, 36, 0.05)".to_string(),
            // banner_cube_border_color: "rgba(101, 109, 74, 0.6)".to_string(),
            // banner_cube_border_color: "#7f4f24".to_string(),
            banner_cube_border_color: "rgba(127, 79, 36, 0.4)".to_string(),
            // banner_sphere_color: "rgba(88, 47, 14, 0.1)".to_string(),
            banner_sphere_color: "rgba(194, 197, 170, 0.1)".to_string(),
            banner_color_one: "#c2c5aa".to_string(),
            banner_color_two: "#c2c5aa".to_string(),
            banner_color_three: "#c2c5aa".to_string(),
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            // gradient_dark: "#1d2118".to_string(),
            // gradient_light: "#333d29".to_string(),
            // navbar: "#414833".to_string(),
            // text_default: "#936639".to_string(),
            // text_invert: "#582f0e".to_string(),
            // text_colored: "#34a0a4".to_string(),
            gradient_dark: "#0b0c10".to_string(),
            gradient_light: "#1f2833".to_string(),
            paper: "#0b0c10".to_string(),
            paper_secondary: "#1f2833".to_string(),
            navbar: "#116466".to_string(),
            sidebar: "#133a45".to_string(),
            text_default: "#d1e8e2".to_string(),
            text_invert: "#1f2833".to_string(),
            text_colored: "#d1e8e2".to_string(),
            h1: "".to_string(),
            h2: "".to_string(),
            h3: "".to_string(),
            h4: "".to_string(),
            h5: "".to_string(),
            h6: "".to_string(),
            hr: "".to_string(),
            link: "#7f4f24".to_string(),
            link_hover: "#582f0e".to_string(),
            link_visited: "#3c1642".to_string(),
            panel_color_primary: "".to_string(),
            panel_color_secondary: "".to_string(),
            panel_color_dark: "".to_string(),
            scroll_bar: "#333d29".to_string(),
            scroll_bar_hover: "#132a13".to_string(),
            banner_cube_color: "rgba(24, 78, 119, 0.006)".to_string(),
            banner_cube_border_color: "rgba(22, 138, 173, 0.2)".to_string(),
            banner_sphere_color: "rgba(0, 0, 0, 0.05)".to_string(),
            banner_color_one: "#52b69a".to_string(),
            banner_color_two: "#99d98c".to_string(),
            banner_color_three: "#d9ed92".to_string(),
        });

        match self {
            ThemeKind::Dark => &DARK_THEME,
            ThemeKind::Light => &LIGHT_THEME,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeContext {
    inner: UseStateHandle<ThemeKind>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
        Self { inner }
    }

    pub fn toggle(&self) {
        match &*self.inner {
            ThemeKind::Dark => self.inner.set(ThemeKind::Light),
            ThemeKind::Light => self.inner.set(ThemeKind::Dark),
        }
    }

    pub fn kind(&self) -> ThemeKind {
        (*self.inner).clone()
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        self.inner.current()
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ThemeProviderProps {
    pub children: Children,
}

#[styled_component]
pub(crate) fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme_kind = use_state(|| ThemeKind::Light);

    let theme_ctx = ThemeContext::new(theme_kind);

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}
#[hook]
pub(crate) fn use_theme() -> ThemeContext {
    use yew::use_context;
    use_context::<ThemeContext>().unwrap()
}
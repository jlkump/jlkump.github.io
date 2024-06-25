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
    pub text_dark: String,
    pub text_invert: String,
    pub text_colored: String,
    pub text_highlight: String,
    pub h1: String,
    pub h2: String,
    pub h3: String,
    pub h4: String,
    pub h5: String,
    pub h6: String,
    pub hr: String,
    pub link: String,
    pub link_hover: String,
    pub link_invert: String,
    pub link_hover_invert: String,
    pub panel_color_primary: String,
    pub panel_color_secondary: String,
    pub panel_color_dark: String,
    pub panel_color_accent: String,
    pub scroll_bar: String,
    pub scroll_bar_hover: String,
    pub button: String,
    pub button_press: String,
    pub banner_cube_color: String,
    pub banner_cube_border_color: String,
    pub banner_sphere_color: String,
    pub banner_color_one: String,
    pub banner_color_two: String,
    pub banner_color_three: String,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ThemeKind {
    Green,
}

impl ImplicitClone for ThemeKind {}

impl ThemeKind {
    pub fn current(&self) -> &Theme {
        static GREEN_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            gradient_dark: "#d6b99c".to_string(),
            gradient_light: "#c2c5aa".to_string(),
            paper: "#c2c5aa".to_string(),
            paper_secondary: "#b6ad90".to_string(),
            navbar: "#414833".to_string(),
            sidebar: "#b6ad90".to_string(),
            text_default: "#313621".to_string(),
            text_dark: "#1d2118".to_string(),
            text_invert: "#c2c5aa".to_string(),
            text_colored: "#224540".to_string(),
            text_highlight: "#332403".to_string(),
            h1: "#582f0e".to_string(),
            h2: "#332403".to_string(),
            h3: "#23331b".to_string(),
            h4: "#333d29".to_string(),
            h5: "#132a13".to_string(),
            h6: "#081c15".to_string(),
            hr: "#132a13".to_string(),
            link: "#582f0e".to_string(),
            link_hover: "#332403".to_string(),
            link_invert: "#7f4f24".to_string(),
            link_hover_invert: "#936639".to_string(),
            panel_color_primary: "#656d4a".to_string(),
            panel_color_secondary: "#545c3b".to_string(),
            panel_color_dark: "#333d29".to_string(),
            panel_color_accent: "#132a13".to_string(),
            scroll_bar: "#333d29".to_string(),
            scroll_bar_hover: "#132a13".to_string(),
            button: "#333d29".to_string(),
            button_press: "#132a13".to_string(),
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

        // static BLUE_THEME: Lazy<Theme> = Lazy::new(|| Theme {
        //     gradient_dark: "#0b0c10".to_string(),
        //     gradient_light: "#1f2833".to_string(),
        //     paper: "#0b0c10".to_string(),
        //     paper_secondary: "#1f2833".to_string(),
        //     navbar: "#116466".to_string(),
        //     sidebar: "#133a45".to_string(),
        //     text_default: "#d1e8e2".to_string(),
        //     text_dark: "#0b0c10".to_string(),
        //     text_invert: "#1f2833".to_string(),
        //     text_colored: "#d1e8e2".to_string(),
        //     text_highlight: "#332403".to_string(),
        //     h1: "#34a0a4".to_string(),
        //     h2: "#34a0a4".to_string(),
        //     h3: "#34a0a4".to_string(),
        //     h4: "#34a0a4".to_string(),
        //     h5: "#34a0a4".to_string(),
        //     h6: "#34a0a4".to_string(),
        //     hr: "#34a0a4".to_string(),
        //     link: "#7f4f24".to_string(),
        //     link_hover: "#582f0e".to_string(),
        //     link_invert: "#7f4f24".to_string(),
        //     link_hover_invert: "#582f0e".to_string(),
        //     link_visited: "#3c1642".to_string(),
        //     panel_color_primary: "#1f2833".to_string(),
        //     panel_color_secondary: "#1f2833".to_string(),
        //     panel_color_dark: "#1f2833".to_string(),
        //     panel_color_accent: "#132a13".to_string(),
        //     scroll_bar: "#133a45".to_string(),
        //     scroll_bar_hover: "#133a45".to_string(),
        //     button: "#133a45".to_string(),
        //     button_press: "#0b0c10".to_string(),
        //     banner_cube_color: "rgba(24, 78, 119, 0.006)".to_string(),
        //     banner_cube_border_color: "rgba(22, 138, 173, 0.2)".to_string(),
        //     banner_sphere_color: "rgba(0, 0, 0, 0.05)".to_string(),
        //     banner_color_one: "#52b69a".to_string(),
        //     banner_color_two: "#99d98c".to_string(),
        //     banner_color_three: "#d9ed92".to_string(),
        // });

        match self {
            ThemeKind::Green => &GREEN_THEME,
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
    let theme_kind = use_state(|| ThemeKind::Green);

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
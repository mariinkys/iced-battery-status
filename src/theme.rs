use iced::{
    widget::{button, container},
    Border, Shadow, Theme, Vector,
};

// Button Styling
#[allow(dead_code)]
pub enum ButtonStyle {
    Standard,
    ThemeButton,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(match self {
                Self::Standard => iced::Color::from_rgb(0.059, 0.463, 0.702),
                Self::ThemeButton => iced::Color::default(),
            })),
            text_color: {
                if style == &Theme::Light {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::BLACK,
                    }
                } else {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::WHITE,
                    }
                }
            },
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::ThemeButton => Border::default(),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color: iced::Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                Self::ThemeButton => Shadow::default(),
            },
            ..Default::default()
        }
    }
}

// Container Styling
pub struct ContainerStyle;
impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Default::default(),
            background: None,
            border: Border::with_radius(5),
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            },
        }
    }
}

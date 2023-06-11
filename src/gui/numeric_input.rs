use iced::widget::{button, row, slider, text, text_input};
use iced::Alignment;
use iced_lazy::{self, Component};
use iced_native::Element;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct NumericInput<T, Message>
where
    T: Clone,
{
    label: String,
    value: T,
    value_string: String,
    range: RangeInclusive<T>,
    step: T,
    text_size: u16,
    width: u16,
    spacing: u16,
    decimals: u8,
    on_release: Box<dyn Fn(T) -> Message>,
}

#[derive(Debug, Clone)]
pub enum Event<T> {
    SliderChanged(T),
    SliderReleased,
    TextChanged(String),
    TextSubmitted,
}

impl<T, Message> NumericInput<T, Message>
where
    T: Clone + std::fmt::Display,
{
    pub fn new(
        label: String,
        value: T,
        range: RangeInclusive<T>,
        step: T,
        decimals: u8,
        on_release: impl Fn(T) -> Message + 'static,
    ) -> Self {
        let n = match decimals {
            0 => format!("{value:0.0}"),
            1 => format!("{value:0.1}"),
            _ => format!("{value:0.2}"),
        };
        Self {
            label,
            value_string: n,
            value,
            range,
            step,
            text_size: 15,
            width: 150,
            spacing: 10,
            decimals,
            on_release: Box::new(on_release),
        }
    }

    pub fn text_size(self, text_size: u16) -> Self {
        Self { text_size, ..self }
    }

    pub fn width(self, width: u16) -> Self {
        Self { width, ..self }
    }

    pub fn spacing(self, spacing: u16) -> Self {
        Self { spacing, ..self }
    }

    pub fn decimals(self, decimals: u8) -> Self {
        Self { decimals, ..self }
    }
}

impl<'a, T, Message, Renderer> Component<Message, Renderer> for NumericInput<T, Message>
where
    T: Copy
        + From<u8>
        + FromStr
        + PartialOrd
        + num_traits::FromPrimitive
        + std::fmt::Display
        + 'static,
    f64: From<T>,
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme: button::StyleSheet + text::StyleSheet + slider::StyleSheet,
    <<Renderer as iced_native::Renderer>::Theme as iced::widget::text::StyleSheet>::Style:
        From<iced::Color>,
    <Renderer as iced_native::Renderer>::Theme: iced::widget::text_input::StyleSheet,
    Message: 'a + Clone,
{
    type State = ();
    type Event = Event<T>;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::SliderChanged(v) => {
                self.value = v;
                let n = match self.decimals {
                    0 => format!("{v:0.0}"),
                    1 => format!("{v:0.1}"),
                    _ => format!("{v:0.2}"),
                };
                self.value_string = n;
                None
            }
            Event::SliderReleased => Some((self.on_release)(self.value)),
            Event::TextChanged(t) => {
                self.value_string = t;
                None
            }
            Event::TextSubmitted => {
                if let Ok(v) = self.value_string.parse::<T>() {
                    self.value = v;
                    Some((self.on_release)(self.value))
                } else {
                    None
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> iced_native::Element<'_, Self::Event, Renderer> {
        let r = row![text(self.label.clone()).size(self.text_size)];
        iced::widget::column![
            r.spacing(self.text_size),
            row![
                slider(self.range.clone(), self.value, Event::SliderChanged)
                    .on_release(Event::SliderReleased)
                    .step(self.step)
                    .width(self.width),
                text_input("", self.value_string.as_str())
                    .on_input(Event::TextChanged)
                    .on_submit(Event::TextSubmitted)
                    .size(15)
                    .width(45)
            ]
            .spacing(self.spacing)
            .align_items(Alignment::Center)
        ]
        .spacing(self.spacing)
        .into()
    }
}

impl<'a, T, Message, Renderer> From<NumericInput<T, Message>> for Element<'a, Message, Renderer>
where
    T: Copy
        + From<u8>
        + FromStr
        + PartialOrd
        + num_traits::FromPrimitive
        + std::fmt::Display
        + 'static,
    f64: From<T>,
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme: button::StyleSheet + text::StyleSheet + slider::StyleSheet,
    <<Renderer as iced_native::Renderer>::Theme as iced::widget::text::StyleSheet>::Style:
        From<iced::Color>,
    <Renderer as iced_native::Renderer>::Theme: iced::widget::text_input::StyleSheet,
    Message: 'a + Clone,
{
    fn from(ni: NumericInput<T, Message>) -> Self {
        iced_lazy::component(ni)
    }
}

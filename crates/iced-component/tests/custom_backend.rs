//! Verifies that an external crate can implement and compose adapter backends.

use iced::{Element, widget::Space};
use iced_component::{
    backend::{ButtonBackend, ButtonViewBackend, ThemeBackend},
    button::{AdaptiveButton, ButtonEvent, ButtonOutcome},
    context::{AdapterContext, AdapterUpdateCx, AdapterViewCx, BackendSelection, ColorScheme},
    core::anim::{MotionError, MotionRuntime},
};

struct CustomBackend;

#[derive(Clone)]
struct CustomContext {
    scheme: ColorScheme,
    reduce_motion: bool,
}

struct CustomUpdateCx<'a> {
    _runtime: &'a mut MotionRuntime,
    context: &'a mut CustomContext,
}

struct CustomViewCx<'a> {
    _runtime: &'a MotionRuntime,
    _context: &'a CustomContext,
}

impl ThemeBackend for CustomBackend {
    type Context = CustomContext;
    type UpdateCx<'a> = CustomUpdateCx<'a>;
    type ViewCx<'a> = CustomViewCx<'a>;

    fn update_cx<'a>(
        runtime: &'a mut MotionRuntime,
        context: &'a mut Self::Context,
    ) -> Self::UpdateCx<'a> {
        CustomUpdateCx {
            _runtime: runtime,
            context,
        }
    }

    fn view_cx<'a>(runtime: &'a MotionRuntime, context: &'a Self::Context) -> Self::ViewCx<'a> {
        CustomViewCx {
            _runtime: runtime,
            _context: context,
        }
    }

    fn color_scheme(context: &Self::Context) -> ColorScheme {
        context.scheme
    }

    fn reduce_motion(context: &Self::Context) -> bool {
        context.reduce_motion
    }

    fn set_color_scheme(cx: &mut Self::UpdateCx<'_>, color_scheme: ColorScheme) -> bool {
        let changed = cx.context.scheme != color_scheme;
        cx.context.scheme = color_scheme;
        changed
    }

    fn set_reduce_motion(cx: &mut Self::UpdateCx<'_>, reduce_motion: bool) {
        cx.context.reduce_motion = reduce_motion;
    }
}

#[derive(Default)]
struct CustomButton {
    disabled: bool,
    registered: bool,
    events: usize,
}

struct CustomButtonView<'a, Message> {
    content: Element<'a, Message>,
    mapper: Option<Box<dyn Fn(ButtonEvent) -> Message + 'a>>,
}

impl<'a, Message> ButtonViewBackend<'a, Message> for CustomButtonView<'a, Message>
where
    Message: Clone + 'a,
{
    fn content(mut self, content: Element<'a, Message>) -> Self {
        self.content = content;
        self
    }

    fn on_event<F>(mut self, mapper: F) -> Self
    where
        F: Fn(ButtonEvent) -> Message + 'a,
    {
        self.mapper = Some(Box::new(mapper));
        self
    }
}

impl<'a, Message> From<CustomButtonView<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(view: CustomButtonView<'a, Message>) -> Self {
        view.content
    }
}

impl ButtonBackend for CustomBackend {
    type Button = CustomButton;
    type View<'a, Message>
        = CustomButtonView<'a, Message>
    where
        Message: Clone + 'a;

    fn disabled(mut button: Self::Button, disabled: bool) -> Self::Button {
        button.disabled = disabled;
        button
    }

    fn register(button: &mut Self::Button, _cx: &mut Self::UpdateCx<'_>) {
        button.registered = true;
    }

    fn sync(button: &mut Self::Button, _cx: &mut Self::UpdateCx<'_>) -> Result<bool, MotionError> {
        Ok(button.registered)
    }

    fn set_disabled(
        button: &mut Self::Button,
        disabled: bool,
        _cx: &mut Self::UpdateCx<'_>,
    ) -> Result<bool, MotionError> {
        let changed = button.disabled != disabled;
        button.disabled = disabled;
        Ok(changed)
    }

    fn update_event(
        button: &mut Self::Button,
        event: ButtonEvent,
        _cx: &mut Self::UpdateCx<'_>,
    ) -> Result<ButtonOutcome, MotionError> {
        button.events += 1;
        Ok(if event == ButtonEvent::Pressed && !button.disabled {
            ButtonOutcome::Activated
        } else {
            ButtonOutcome::None
        })
    }

    fn view<'a, Message>(
        _button: &'a Self::Button,
        _cx: &Self::ViewCx<'_>,
    ) -> Self::View<'a, Message>
    where
        Message: Clone + 'a,
    {
        CustomButtonView {
            content: Space::new().into(),
            mapper: None,
        }
    }

    fn is_registered(button: &Self::Button) -> bool {
        button.registered
    }
}

#[test]
fn external_backend_can_supply_an_entire_adapter_pair() {
    type Context = AdapterContext<CustomBackend, CustomBackend>;
    type UpdateCx<'a> = AdapterUpdateCx<'a, CustomBackend, CustomBackend>;
    type ViewCx<'a> = AdapterViewCx<'a, CustomBackend, CustomBackend>;
    type Button = AdaptiveButton<CustomBackend, CustomBackend>;

    let mut runtime = MotionRuntime::new();
    let mut context = Context::from_backends(
        BackendSelection::First,
        CustomContext {
            scheme: ColorScheme::Light,
            reduce_motion: false,
        },
        CustomContext {
            scheme: ColorScheme::Dark,
            reduce_motion: false,
        },
    );
    let mut button = Button::from_backends(CustomButton::default(), CustomButton::default());

    let mut cx = UpdateCx::new(&mut runtime, &mut context);
    button.register(&mut cx);
    cx.set_selection(BackendSelection::Second);
    let outcome = button.update_event(ButtonEvent::Pressed, &mut cx).unwrap();

    assert!(button.is_registered());
    assert_eq!(button.first().events, 1);
    assert_eq!(button.second().events, 1);
    assert_eq!(outcome, ButtonOutcome::Activated);
    assert_eq!(context.color_scheme(), ColorScheme::Dark);

    let _: Element<'_, ButtonEvent> = button
        .view(&ViewCx::new(&runtime, &context))
        .on_event(|event| event)
        .into();
}

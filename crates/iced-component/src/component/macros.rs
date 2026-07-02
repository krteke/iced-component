/// Registers multiple component motion handles against one update context.
///
/// Syntax:
///
/// ```
/// # use iced_component::button::Button;
/// # use iced_component::anim::MotionRuntime;
/// # use iced_component::component::{ComponentContext, ComponentUpdateCx};
/// # let mut runtime = MotionRuntime::new();
/// # let mut context = ComponentContext::default();
/// # let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
/// # let mut save = Button::suggested("Save");
/// # let mut cancel = Button::standard("Cancel");
/// iced_component::register_components!(cx, [save, cancel]);
/// ```
#[macro_export]
macro_rules! register_components {
    ($cx:expr, [$($component:expr),* $(,)?]) => {{
        let cx = &mut $cx;
        $(
            ($component).register(cx);
        )*
    }};
}

/// Synchronizes multiple components against one update context.
///
/// The macro returns `Result<bool, MotionError>`, where the boolean is `true`
/// when at least one component submitted a runtime motion update.
#[macro_export]
macro_rules! sync_components {
    ($cx:expr, [$($component:expr),* $(,)?]) => {{
        || -> Result<bool, $crate::anim::MotionError> {
            let cx = &mut $cx;
            let mut changed = false;
            $(
                changed |= ($component).sync(cx)?;
            )*
            Ok(changed)
        }()
    }};
}

#[cfg(test)]
mod tests {
    use aura_anim::prelude::{Duration, MotionRuntime};

    use crate::{
        button::Button,
        component::{ComponentContext, ComponentUpdateCx, ComponentViewCx},
        surface::Surface,
    };
    use spectrum_theme::Color;

    #[test]
    fn register_components_registers_each_component_once() {
        let mut runtime = MotionRuntime::new();
        let mut button = Button::suggested("Save");
        let mut surface = Surface::raised();

        let mut context = ComponentContext::default();
        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            crate::register_components!(cx, [button, surface]);
            crate::register_components!(cx, [button, surface]);
        }

        assert_eq!(runtime.motion_count(), 2);
    }

    #[test]
    fn sync_components_returns_whether_registered_components_changed() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::default();
        let mut button = Button::suggested("Save");
        let mut surface = Surface::raised();

        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        let changed = crate::sync_components!(cx, [button, surface]).unwrap();

        assert!(!changed);

        crate::register_components!(cx, [button, surface]);

        let changed = crate::sync_components!(cx, [button, surface]).unwrap();

        assert!(changed);
    }

    #[test]
    fn sync_components_uses_current_theme_revision() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::default();
        let mut button = Button::standard("Save");

        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            crate::register_components!(cx, [button]);
        }

        let patched_bg = Color::new_rgba(221, 238, 255, 255);
        context.patch_theme(|theme| theme.button.standard_filled.idle.bg = patched_bg);

        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            assert!(crate::sync_components!(cx, [button]).unwrap());
        }
        runtime.tick(Duration::from_millis(200.0));

        let cx = ComponentViewCx::new(&runtime, &context);
        assert_eq!(button.snapshot(&cx).unwrap().style.background, patched_bg);
        assert_eq!(
            button.motion_value(&runtime).unwrap().unwrap().tokens.bg,
            patched_bg
        );
    }
}

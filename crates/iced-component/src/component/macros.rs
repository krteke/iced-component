/// Registers multiple component motion handles against one runtime.
///
/// Syntax:
///
/// ```
/// # use iced_component::anim::MotionRuntime;
/// # use iced_component::button::Button;
/// # let mut runtime = MotionRuntime::new();
/// # let mut save = Button::suggested("Save");
/// # let mut cancel = Button::standard("Cancel");
/// iced_component::register_components!(runtime, [save, cancel]);
/// ```
#[macro_export]
macro_rules! register_components {
    ($runtime:expr, [$($component:expr),* $(,)?]) => {{
        let runtime = &mut $runtime;
        $(
            ($component).register(runtime);
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
    use aura_anim::prelude::MotionRuntime;

    use crate::{
        button::Button,
        component::{ComponentContext, ComponentUpdateCx},
        surface::Surface,
    };

    #[test]
    fn register_components_registers_each_component_once() {
        let mut runtime = MotionRuntime::new();
        let mut button = Button::suggested("Save");
        let mut surface = Surface::raised();

        crate::register_components!(runtime, [button, surface]);
        crate::register_components!(runtime, [button, surface]);

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

        crate::register_components!(runtime, [button, surface]);

        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        let changed = crate::sync_components!(cx, [button, surface]).unwrap();

        assert!(changed);
    }
}

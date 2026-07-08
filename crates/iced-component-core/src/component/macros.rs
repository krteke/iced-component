/// Registers multiple component motion handles against one update context.
///
/// Syntax:
///
/// ```
/// # use iced_component_core::anim::MotionRuntime;
/// # use iced_component_core::component::{ComponentContext, ComponentUpdateCx};
/// # struct Component;
/// # impl Component {
/// #     fn register(&mut self, _cx: &mut ComponentUpdateCx<'_>) {}
/// # }
/// # let mut runtime = MotionRuntime::new();
/// # let mut context = ComponentContext::default();
/// # let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
/// # let mut save = Component;
/// # let mut cancel = Component;
/// iced_component_core::register_components!(cx, [save, cancel]);
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
    use aura_anim::prelude::MotionRuntime;

    use crate::component::{ComponentContext, ComponentUpdateCx};

    #[derive(Debug, Default)]
    struct DummyComponent {
        registered: usize,
        syncs: usize,
    }

    impl DummyComponent {
        fn register(&mut self, _cx: &mut ComponentUpdateCx<'_>) {
            self.registered += 1;
        }

        #[allow(clippy::unnecessary_wraps)]
        fn sync(
            &mut self,
            _cx: &mut ComponentUpdateCx<'_>,
        ) -> Result<bool, crate::anim::MotionError> {
            self.syncs += 1;
            Ok(self.registered > 0)
        }
    }

    #[test]
    fn register_components_calls_each_component() {
        let mut runtime = MotionRuntime::new();
        let mut first = DummyComponent::default();
        let mut second = DummyComponent::default();

        let mut context = ComponentContext::default();
        {
            let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
            crate::register_components!(cx, [first, second]);
        }

        assert_eq!(first.registered, 1);
        assert_eq!(second.registered, 1);
    }

    #[test]
    fn sync_components_returns_whether_any_component_changed() {
        let mut runtime = MotionRuntime::new();
        let mut context = ComponentContext::default();
        let mut first = DummyComponent::default();
        let mut second = DummyComponent::default();

        let mut cx = ComponentUpdateCx::new(&mut runtime, &mut context);
        let changed = crate::sync_components!(cx, [first, second]).unwrap();

        assert!(!changed);

        crate::register_components!(cx, [first]);

        let changed = crate::sync_components!(cx, [first, second]).unwrap();

        assert!(changed);
        assert_eq!(first.syncs, 2);
        assert_eq!(second.syncs, 2);
    }
}

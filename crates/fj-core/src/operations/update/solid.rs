use crate::{
    objects::{Shell, Solid},
    storage::Handle,
};

/// Update a [`Solid`]
pub trait UpdateSolid {
    /// Add a shell to the solid
    #[must_use]
    fn add_shells(
        &self,
        shells: impl IntoIterator<Item = Handle<Shell>>,
    ) -> Self;

    /// Update a shell of the solid
    ///
    /// # Panics
    ///
    /// Uses [`Handles::update`] internally, and panics for the same reasons.
    ///
    /// [`Handles::update`]: crate::objects::Handles::update
    #[must_use]
    fn update_shell(
        &self,
        handle: &Handle<Shell>,
        update: impl FnOnce(&Handle<Shell>) -> Handle<Shell>,
    ) -> Self;

    /// Replace a shell of the solid
    ///
    /// This is a more general version of [`UpdateSolid::update_shell`] which
    /// can replace a single edge with multiple others.
    ///
    /// # Panics
    ///
    /// Uses [`Handles::replace`] internally, and panics for the same reasons.
    ///
    /// [`Handles::replace`]: crate::objects::Handles::replace
    #[must_use]
    fn replace_shell<const N: usize>(
        &self,
        handle: &Handle<Shell>,
        replace: impl FnOnce(&Handle<Shell>) -> [Handle<Shell>; N],
    ) -> Self;
}

impl UpdateSolid for Solid {
    fn add_shells(
        &self,
        shells: impl IntoIterator<Item = Handle<Shell>>,
    ) -> Self {
        let shells = self.shells().iter().cloned().chain(shells);
        Solid::new(shells)
    }

    fn update_shell(
        &self,
        handle: &Handle<Shell>,
        update: impl FnOnce(&Handle<Shell>) -> Handle<Shell>,
    ) -> Self {
        let shells = self.shells().update(handle, update);
        Solid::new(shells)
    }

    fn replace_shell<const N: usize>(
        &self,
        handle: &Handle<Shell>,
        replace: impl FnOnce(&Handle<Shell>) -> [Handle<Shell>; N],
    ) -> Self {
        let shells = self.shells().replace(handle, replace);
        Solid::new(shells)
    }
}

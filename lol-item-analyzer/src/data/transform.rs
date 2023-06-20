#[cfg(test)]
use mockall::automock;

/// A Transformer transforms some arbitrary context Ctx to a new state.
#[cfg_attr(test, automock)]
pub trait Transformer<Ctx> {
    /// Transform a context, ctx, to a new state. Modifications are done on the original given
    /// context.
    fn transform(&self, ctx: &mut Ctx);
}

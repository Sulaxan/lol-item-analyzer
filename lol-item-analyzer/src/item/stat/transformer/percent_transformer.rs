use super::{StatTransformer, StatTransformContext};

pub struct StatPercentTransformer;

impl StatTransformer for StatPercentTransformer {
    fn transform(&self, ctx: &mut StatTransformContext) {
        ctx.stats.borrow_mut().iter_mut().for_each(|(id, stat)| {
            if id.contains("Percent") {
                stat.is_percent = true;
            }
        });
    }
}

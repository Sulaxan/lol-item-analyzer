use crate::data::transform::Transformer;

use super::StatTransformContext;

pub struct StatPercentTransformer;

impl Transformer<StatTransformContext> for StatPercentTransformer {
    fn transform(&self, ctx: &mut StatTransformContext) {
        ctx.stats.borrow_mut().iter_mut().for_each(|(id, stat)| {
            if id.contains("Percent") {
                stat.is_percent = true;
            }
        });
    }
}

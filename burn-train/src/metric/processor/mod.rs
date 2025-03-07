mod base;
mod full;
mod metrics;
mod minimal;

pub use base::*;
pub(crate) use full::*;
pub(crate) use metrics::*;

#[cfg(test)]
pub(crate) use minimal::*;

#[cfg(test)]
pub(crate) mod test_utils {
    use crate::metric::{
        processor::{Event, EventProcessor, LearnerItem, MinimalEventProcessor},
        Adaptor, LossInput,
    };
    use burn_core::tensor::{backend::Backend, ElementConversion, Tensor};

    impl<B: Backend> Adaptor<LossInput<B>> for f64 {
        fn adapt(&self) -> LossInput<B> {
            LossInput::new(Tensor::from_data([self.elem()]))
        }
    }

    pub(crate) fn process_train(
        processor: &mut MinimalEventProcessor<f64, f64>,
        value: f64,
        epoch: usize,
    ) {
        let dummy_progress = burn_core::data::dataloader::Progress {
            items_processed: 1,
            items_total: 10,
        };
        let num_epochs = 3;
        let dummy_iteration = 1;

        processor.process_train(Event::ProcessedItem(LearnerItem::new(
            value,
            dummy_progress,
            epoch,
            num_epochs,
            dummy_iteration,
            None,
        )));
    }

    pub(crate) fn end_epoch(processor: &mut MinimalEventProcessor<f64, f64>, epoch: usize) {
        processor.process_train(Event::EndEpoch(epoch));
        processor.process_valid(Event::EndEpoch(epoch));
    }
}

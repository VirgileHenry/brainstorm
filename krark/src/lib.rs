mod args;
mod recap_display;
mod result;

pub use result::KrarkResult;

use result::FailedResult;
use result::KrarkResultStatus;
use result::PanickedResult;
use result::PassedResult;

pub struct KrarkHarness {
    test_args: libtest_mimic::Arguments,
    krark_args: args::KrarkArgs,
    name: String,
}

impl KrarkHarness {
    pub fn new(name: String) -> KrarkHarness {
        KrarkHarness {
            test_args: libtest_mimic::Arguments::from_args(),
            krark_args: args::KrarkArgs::default(),
            name,
        }
    }

    pub fn args(&mut self) -> &mut args::KrarkArgs {
        &mut self.krark_args
    }

    pub fn run<R: Fn(&mtg_cardbase::Card, KrarkResult) -> KrarkResult + std::panic::RefUnwindSafe + Sync>(
        &mut self,
        test_func: R,
    ) {
        let cards = mtg_cardbase::AllCardsIter::new();
        let mut recap = KrarkRecap::new(cards.len());

        let results: Vec<_> = cards
            .iter()
            .map(
                |card| match std::panic::catch_unwind(|| test_func(card, KrarkResult::new(card.name.clone()))) {
                    Ok(result) => result,
                    Err(payload) => KrarkResult::from_panic_payload(card.name.clone(), payload),
                },
            )
            .collect();

        for result in results.into_iter() {
            recap.add_result(result);
        }

        match recap_display::output_recap(&self, recap) {
            Ok(_) => { /* all good */ }
            Err(e) => println!("Failed to output recap: {e}"),
        }
    }

    pub fn run_filter<
        F: Fn(&mtg_cardbase::Card) -> bool + Sync + Send,
        R: Fn(&mtg_cardbase::Card, KrarkResult) -> KrarkResult + std::panic::RefUnwindSafe + Sync,
    >(
        &mut self,
        filter: F,
        test_func: R,
    ) {
        let cards = mtg_cardbase::AllCardsIter::new();
        let mut recap = KrarkRecap::new(cards.len());

        let results: Vec<_> = cards
            .iter()
            .filter(|card| filter(card))
            .map(
                |card| match std::panic::catch_unwind(|| test_func(card, KrarkResult::new(card.name.clone()))) {
                    Ok(result) => result,
                    Err(payload) => KrarkResult::from_panic_payload(card.name.clone(), payload),
                },
            )
            .collect();

        for result in results.into_iter() {
            recap.add_result(result);
        }

        match recap_display::output_recap(&self, recap) {
            Ok(_) => { /* all good */ }
            Err(e) => println!("Failed to output recap: {e}"),
        }
    }
}

struct KrarkRecap {
    passed: Vec<PassedResult>,
    failed: Vec<FailedResult>,
    panicked: Vec<PanickedResult>,
}

impl KrarkRecap {
    fn new(capacity: usize) -> KrarkRecap {
        KrarkRecap {
            passed: Vec::with_capacity(capacity),
            failed: Vec::with_capacity(capacity),
            panicked: Vec::with_capacity(capacity),
        }
    }
    fn add_result(&mut self, result: KrarkResult) {
        match result.status {
            KrarkResultStatus::Passed(passed) => self.passed.push(passed),
            KrarkResultStatus::Failed(failed) => self.failed.push(failed),
            KrarkResultStatus::Panicked(panicked) => self.panicked.push(panicked),
            KrarkResultStatus::Skipped => {}
        }
    }
}

use std::fmt::Display;

use chrono::{DateTime, TimeDelta, Utc};
use dioxus::prelude::*;
use gloo_timers::callback::Interval;

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UtcClock;

impl UtcClock {
    pub fn new() -> Self {
        Self
    }
}

impl Clock for UtcClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Stopwatch<C: Clock> {
    clock: C,
    start: Option<DateTime<Utc>>,
    elapsed: TimeDelta,
    total: TimeDelta,
}

impl<C: Clock> Stopwatch<C> {
    fn new(clock: C, total: TimeDelta) -> Self {
        Self {
            clock,
            start: None,
            elapsed: TimeDelta::zero(),
            total,
        }
    }

    fn start(&mut self) {
        if self.start.is_none() {
            self.start = Some(self.clock.now());
        }
    }

    fn stop(&mut self) {
        if let Some(start) = self.start {
            self.elapsed += self.clock.now() - start;
            self.start = None;
        }
    }

    fn toggle(&mut self) {
        if self.start.is_some() {
            self.stop();
        } else {
            self.start();
        }
    }

    fn lap(&mut self) -> TimeDelta {
        let elapsed = self.elapsed();
        self.elapsed = TimeDelta::zero();
        if self.start.is_some() {
            self.start = Some(self.clock.now());
        }
        elapsed
    }

    fn running(&self) -> bool {
        self.start.is_some()
    }

    fn elapsed(&self) -> TimeDelta {
        if let Some(start) = self.start {
            self.elapsed + (self.clock.now() - start)
        } else {
            self.elapsed
        }
    }

    fn remaining(&self) -> TimeDelta {
        self.total - self.elapsed()
    }

    fn progress(&self) -> f32 {
        let elapsed = self.elapsed();
        if elapsed >= self.total {
            1.0
        } else if elapsed <= TimeDelta::zero() {
            0.0
        } else {
            elapsed.as_seconds_f32() / self.total.as_seconds_f32()
        }
    }
}

impl Clone for Stopwatch<UtcClock> {
    fn clone(&self) -> Self {
        Self {
            clock: UtcClock::new(),
            start: self.start,
            elapsed: self.elapsed,
            total: self.total,
        }
    }
}

impl Display for Stopwatch<UtcClock> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seconds = self.remaining().num_seconds().abs();
        let minutes = seconds / 60;
        write!(f, "{:02}:{:02}", minutes, seconds % 60)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, ops::AddAssign, rc::Rc};

    use chrono::TimeZone;

    use super::*;

    #[derive(Debug, Clone)]
    struct MockClock {
        now: Rc<RefCell<DateTime<Utc>>>,
    }

    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            *self.now.borrow()
        }
    }

    impl AddAssign<TimeDelta> for MockClock {
        fn add_assign(&mut self, delta: TimeDelta) {
            *self.now.borrow_mut() += delta;
        }
    }

    #[test]
    fn test_stopwatch_progress() {
        let mut clock = MockClock {
            now: Rc::new(RefCell::new(Utc.timestamp_opt(0, 0).unwrap())),
        };

        let mut sw = Stopwatch::new(clock.clone(), TimeDelta::seconds(2));
        assert_eq!(sw.remaining(), TimeDelta::seconds(2));
        assert_eq!(sw.progress(), 0.0);

        // advance time without starting the stopwatch should not change remaining or progress
        clock += TimeDelta::seconds(1);
        assert_eq!(sw.remaining(), TimeDelta::seconds(2));
        assert_eq!(sw.progress(), 0.0);

        // start the stopwatch and check that it counts down
        sw.start();

        // advance time by a second while running
        clock += TimeDelta::seconds(1);
        assert_eq!(sw.remaining(), TimeDelta::seconds(1));
        assert_eq!(sw.progress(), 0.5);

        // advance time by another second while running
        clock += TimeDelta::seconds(1);
        assert_eq!(sw.remaining(), TimeDelta::zero());
        assert_eq!(sw.progress(), 1.0);

        // advance time by a second after reaching the end
        clock += TimeDelta::seconds(1);
        assert_eq!(sw.remaining(), TimeDelta::seconds(-1));
        assert_eq!(sw.progress(), 1.0);

        // stop the stopwatch and check that it stops counting down
        sw.stop();

        // advance time by a second while stopped
        clock += TimeDelta::seconds(1);
        assert_eq!(sw.remaining(), TimeDelta::seconds(-1));
        assert_eq!(sw.progress(), 1.0);
    }

    #[test]
    fn test_stopwatch_display() {
        todo!("Implement Display trait test for Stopwatch");
    }
}

const TICK_MS: u32 = 1000 / 15;

#[component]
pub fn Timer() -> Element {
    let total = TimeDelta::seconds(10); // TODO: make configurable
    let mut state = use_signal(move || Stopwatch::new(UtcClock::new(), total));

    let current = state.read();
    let time_remain = current.to_string();
    let progress = current.progress();

    use_effect(move || {
        let interval = Interval::new(TICK_MS, move || {
            state.write().running(); // trigger re-render
        });
        interval.forget();
    });

    rsx! {
        div {
            id: "timer-container",
            div {
                id: "timer",
                class: "relative w-full bg-gray-800 h-15 m-1 overflow-hidden rounded",
                div {
                    id: "timer-fill",
                    class: "h-full bg-gradient-to-b from-blue-400 via-blue-600 to-slate-800",
                    style: "width: {progress * 100f32}%",
                }
                span {
                    id: "timer-text",
                    class: "absolute inset-0 flex items-center justify-center text-[1.5em] font-bold text-shadow-md/50",
                    "{time_remain}"
                }
            }
            div {
                id: "timer-controls",
                class: "columns-3 w-full",
                button {
                    id: "timer-toggle",
                    class: "bg-gray-700 w-full text-white rounded h-15 m-1 text-[1.5em] font-bold",
                    onclick: move |_| state.write().toggle(),
                    if current.running() {
                        "\u{23F8}"
                    } else {
                        "\u{23F5}"
                    }
                }
                button {
                    id: "timer-add30s",
                    class: "bg-gray-700 w-full text-white rounded h-15 m-1 text-[1.5em] font-bold",
                    onclick: move |_| {
                        // TODO: add 30 seconds to the total
                    },
                    "+30s"
                }
                button {
                    id: "timer-next",
                    class: "bg-gray-700 w-full text-white rounded h-15 m-1 text-[1.5em] font-bold",
                    onclick: move |_| { state.write().lap(); },
                    "\u{23ED}"
                }
            }
        }
    }
}

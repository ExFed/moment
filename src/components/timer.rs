use chrono::{DateTime, TimeDelta, Utc};
use dioxus::prelude::*;
use gloo_timers::callback::Interval;

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UtcClock;

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

    fn advance(&mut self, delta: TimeDelta) {
        self.elapsed += delta;
    }

    fn reset(&mut self) {
        self.elapsed = TimeDelta::zero();
        self.start = None;
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

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, ops::AddAssign, rc::Rc};

    use chrono::TimeZone;

    use super::*;

    #[derive(Debug, Clone)]
    struct MockClock {
        now: Rc<RefCell<DateTime<Utc>>>,
    }

    impl MockClock {
        fn advance(&self, delta: TimeDelta) {
            *self.now.borrow_mut() += delta;
        }
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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct TimerState {
    millis_total: u32,
    millis_elapsed: i32,
    running: bool,
}

impl TimerState {
    fn new(millis_total: u32) -> Self {
        Self {
            millis_total,
            millis_elapsed: 0,
            running: true,
        }
    }

    fn tick(&mut self, millis_delta: i32) {
        if self.running {
            self.millis_elapsed += millis_delta;
        }
    }

    fn reset(&mut self) {
        self.millis_elapsed = 0;
    }

    fn toggle(&mut self) {
        self.running = !self.running;
    }

    fn millis_remaining(&self) -> i32 {
        self.millis_total as i32 - self.millis_elapsed
    }

    fn progress(&self) -> f32 {
        if self.millis_elapsed >= self.millis_total as i32 {
            1.0
        } else if self.millis_elapsed <= 0 {
            0.0
        } else {
            self.millis_elapsed as f32 / self.millis_total as f32
        }
    }
}

fn fmt_millis(millis: i32) -> String {
    let seconds = millis.abs() / 1000;
    let minutes = seconds / 60;
    format!("{:02}:{:02}", minutes, seconds % 60)
}

const TICK_MS: u32 = 100;

#[component]
pub fn Timer() -> Element {
    let mut state = use_signal(move || TimerState::new(10_000));

    let current = *state.read();
    let time_remain = fmt_millis(current.millis_remaining());
    let progress = current.progress();

    use_effect(move || {
        let interval = Interval::new(TICK_MS, move || {
            state.write().tick(TICK_MS as i32);
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
                button {
                    id: "timer-toggle",
                    class: "bg-gray-700 text-white rounded w-[33%] h-15 m-1 text-[1.5em] font-bold hover:opacity-100 transition-opacity",
                    onclick: move |_| state.write().toggle(),
                    if current.running {
                        "\u{23F8}"
                    } else {
                        "\u{23F5}"
                    }
                }
                button {
                    id: "timer-add30s",
                    class: "bg-gray-700 text-white rounded w-[33%] h-15 m-1 text-[1.5em] font-bold hover:opacity-100 transition-opacity",
                    onclick: move |_| state.write().tick(-30_000),
                    "+30s"
                }
                button {
                    id: "timer-next",
                    class: "bg-gray-700 text-white rounded w-[33%] h-15 m-1 text-[1.5em] font-bold hover:opacity-100 transition-opacity",
                    onclick: move |_| state.write().reset(),
                    "\u{23ED}"
                }
            }
        }
    }
}

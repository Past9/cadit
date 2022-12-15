use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use three_d::Quaternion;

pub trait Interpolate {
    fn interpolate_to(&self, to: Self, t: f32) -> Self;
}
impl Interpolate for Quaternion<f32> {
    fn interpolate_to(&self, to: Self, t: f32) -> Self {
        self.nlerp(to, t)
    }
}

pub struct Interpolation<V: Interpolate + Copy> {
    easing: Easing,
    from: V,
    to: V,
}
impl<V: Interpolate + Copy> Interpolation<V> {
    pub fn linear(a: V, b: V) -> Self {
        Self {
            easing: Easing::Linear,
            from: a,
            to: b,
        }
    }

    pub fn swing(a: V, b: V) -> Self {
        Self {
            easing: Easing::Swing,
            from: a,
            to: b,
        }
    }

    fn at(&self, t: f32) -> V {
        let t = match self.easing {
            Easing::Linear => t,
            Easing::Swing => Self::linear_to_swing(t),
        };

        self.from.interpolate_to(self.to, t)
    }

    fn linear_to_swing(t: f32) -> f32 {
        -(t * std::f32::consts::PI).cos() * 0.5 + 0.5
    }
}

pub enum Easing {
    Linear,
    Swing,
}

pub struct Animation<V>
where
    V: Interpolate + Copy,
{
    interpolation: Interpolation<V>,
    start: Instant,
    end: Instant,
}
impl<V> Animation<V>
where
    V: Interpolate + Copy,
{
    pub fn instant(value: V) -> Self {
        Self {
            interpolation: Interpolation::linear(value, value),
            start: Instant::now(),
            end: Instant::now(),
        }
    }

    pub fn linear(value: V, duration: Duration) -> Self {
        let now = Instant::now();
        Self {
            interpolation: Interpolation::linear(value, value),
            start: now,
            end: now + duration,
        }
    }

    pub fn swing(value: V, duration: Duration) -> Self {
        let now = Instant::now();
        Self {
            interpolation: Interpolation::swing(value, value),
            start: now,
            end: now + duration,
        }
    }

    pub fn current(&self) -> V {
        let duration = self.end - self.start;
        let elapsed = Instant::now() - self.start;
        self.interpolation
            .at(elapsed.as_secs_f32() / duration.as_secs_f32())
    }
}

pub struct AnimatedValue<V>
where
    V: Interpolate + Copy,
{
    queue: VecDeque<Animation<V>>,
    last_value: V,
}
impl<V> AnimatedValue<V>
where
    V: Interpolate + Copy,
{
    pub fn new(value: V) -> Self {
        Self {
            queue: VecDeque::new(),
            last_value: value,
        }
    }

    pub fn has_queued(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn value(&mut self) -> V {
        if !self.queue.is_empty() {
            let now = Instant::now();
            self.queue.retain(|step| {
                if now >= step.start && now <= step.end {
                    self.last_value = step.current();
                    true
                } else if now > step.end {
                    false
                } else {
                    true
                }
            });
        }

        self.last_value
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn set_immediate(&mut self, value: V) {
        self.clear();
        self.last_value = value;
    }

    pub fn push_linear(&mut self, next: V, duration: Duration) {
        self.push_animation(next, duration, Easing::Linear)
    }

    pub fn push_swing(&mut self, next: V, duration: Duration) {
        self.push_animation(next, duration, Easing::Swing)
    }

    fn push_animation(&mut self, next: V, duration: Duration, easing: Easing) {
        let (start_value, start_time) = match self.queue.front() {
            Some(step) => (step.interpolation.to, step.end),
            None => (self.last_value, Instant::now()),
        };

        self.queue.push_back(Animation {
            interpolation: Interpolation {
                easing,
                from: start_value,
                to: next,
            },
            start: start_time,
            end: start_time + duration,
        })
    }
}

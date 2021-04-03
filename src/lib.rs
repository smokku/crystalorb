#![feature(const_fn_floating_point_arithmetic)]
#![feature(map_first_last)]

pub mod channels;
pub mod client;
pub mod command;
pub mod events;
pub mod fixed_timestepper;
pub mod old_new;
pub mod timestamp;
pub mod world;

//#[cfg(not(target_arch = "wasm32"))]
pub mod server;

pub use bevy_networking_turbulence as net;
pub use client::NetworkedPhysicsClientPlugin;

//#[cfg(not(target_arch = "wasm32"))]
pub use server::NetworkedPhysicsServerPlugin;

#[derive(Clone)]
pub struct Config {
    /// Maximum amount of client lag in seconds that the server will compensate for.
    /// A higher number allows a client with a high ping to be able to perform actions
    /// on the world at the correct time from the client's point of view.
    /// The server will only simulate `lag_compensation_latency` seconds in the past,
    /// and let each client predict `lag_compensation_latency` seconds ahead of the server
    /// using the command buffers.
    pub lag_compensation_latency: f32,

    /// When a client receives a snapshot update of the entire world from the server, the client
    /// uses this snapshot to update their simulation. However, immediately displaying the result
    /// of this update will have entities suddenly teleporting to their new destinations. Instead,
    /// we keep simulating the world without the new snapshot information, and slowly fade into the
    /// world with the new snapshot information. We linearly interpolate from the old and new
    /// worlds in `interpolation_latency` seconds.
    pub interpolation_latency: f32,

    pub timestep_seconds: f32,

    pub timestamp_sync_needed_sample_count: usize,

    pub initial_clock_sync_period: f32,

    pub heartbeat_period: f32,

    pub connection_timeout_seconds: f32,

    pub snapshot_send_period: f32,

    pub update_delta_seconds_max: f32,

    pub timestamp_skip_threshold_seconds: f32,

    pub fastforward_max_per_step: usize,

    pub clock_offset_update_factor: f64,
}

impl Config {
    pub const fn new() -> Self {
        Self {
            lag_compensation_latency: 0.3,
            interpolation_latency: 0.2,
            timestep_seconds: 1.0 / 60.0,
            timestamp_sync_needed_sample_count: 32,
            initial_clock_sync_period: 0.2,
            heartbeat_period: 0.7,
            connection_timeout_seconds: 10.0,
            snapshot_send_period: 0.1,
            update_delta_seconds_max: 0.25,
            timestamp_skip_threshold_seconds: 1.0,
            fastforward_max_per_step: 10,
            clock_offset_update_factor: 0.1,
        }
    }
    pub fn lag_compensation_frame_count(&self) -> i16 {
        return (self.lag_compensation_latency / self.timestep_seconds).round() as i16;
    }

    pub fn interpolation_progress_per_frame(&self) -> f32 {
        return self.timestep_seconds / self.interpolation_latency;
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

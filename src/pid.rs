struct PidSystem {
    pid: (i32, i32, i32),
    previous_error: i64,
    integral: i64,
    setpoint: i64,
    error: i64,
    dt: Duration,
    running: bool,

    input: Box<Fn() -> i64>,
    output: Box<Fn(i64)>
}

impl PidSystem {
    pub fn run(mut self) {
        thread::spawn(move || {
            while self.running {
                // User input function could take a non-trivial amount of time
                let span = Duration::span(|| {
                    let pid_out = self.tick(self.input());
                    self.output(pid_out);
                });

                if span >= Duration::zero() {
                    // Unwrapping will not panic, because `span` is garunteed to be greater than 0
                    // here
                    thread::sleep(self.dt - span.to_std().unwrap());
                } else {
                    // We missed an update tick!
                    // TODO: Maybe update proportionally to the amount of time lagging behind?
                }
            }
        });
    }

    pub fn tick(&mut self, input: i64, dt: u64) {
        self.error = self.setpoint - input;
        self.integral = self.integral + self.error * dt;
        let derivative = (self.error - self.previous_error) / dt;
        self.pid.0 * error + self.pid.1 * integral + self.pid.2 * derivative
    }
}

# NeuraKernel AI Driver

This Rust program reads CPU sensors and provides real-time data to the RL agent. It also applies frequency control based on commands from the RL agent.

- Outputs `/tmp/neurakernel_sensors.json`
- Reads `/tmp/neurakernel_action.txt` for actions: -1, 0, +1

## Environment Variables

See the root `.env.example` for configuration options.
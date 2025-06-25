import time
import json
import os
import numpy as np

from ppo_agent import PPOAgent

def getenv(key, default, cast_type=str):
    val = os.environ.get(key, default)
    return cast_type(val)

try:
    from dotenv import load_dotenv
    load_dotenv()
except ImportError:
    pass

SENSOR_PATH = os.environ.get("PYTHON_SENSOR_PATH", os.environ.get("SENSOR_PATH", "/tmp/neurakernel_sensors.json"))
ACTION_PATH = os.environ.get("PYTHON_ACTION_PATH", os.environ.get("ACTION_PATH", "/tmp/neurakernel_action.txt"))
NUM_CORES = getenv("NUM_CORES", 4, int)
EPISODE_LENGTH = getenv("EPISODE_LENGTH", 64, int)
LEARNING_RATE = getenv("LEARNING_RATE", 1e-3, float)
POLL_INTERVAL = getenv("POLL_INTERVAL_MS", 100, int) / 1000.0

def read_state():
    if not os.path.exists(SENSOR_PATH):
        return None
    with open(SENSOR_PATH, "r") as f:
        data = json.load(f)
    state = [data["cpu_temp"]] + data["core_freqs"] + [data["ppt"]]
    return state

def write_action(action):
    with open(ACTION_PATH, "w") as f:
        f.write(str(action))

def reward_fn(state, action):
    temp_penalty = -max(0, state[0] - 70)
    power_penalty = -0.2 * state[-1]
    freq_penalty = -0.01 * np.std(state[1:-1])
    return temp_penalty + power_penalty + freq_penalty

def main():
    input_dim = 1 + NUM_CORES + 1
    agent = PPOAgent(input_dim, lr=LEARNING_RATE)
    prev_state = None
    prev_action = None
    episode_buffer = []

    while True:
        state = read_state()
        if state:
            action = agent.select_action(state)
            print(f"Input state: {state} -> Action: {action}")
            write_action(action)
            if prev_state is not None:
                reward = reward_fn(state, action)
                done = False
                episode_buffer.append((prev_state, prev_action, reward, state, done))
                if len(episode_buffer) >= EPISODE_LENGTH:
                    batch = tuple(np.array(x) for x in zip(*episode_buffer))
                    agent.train_step(batch)
                    episode_buffer.clear()
            prev_state = state
            prev_action = action
        time.sleep(POLL_INTERVAL)

if __name__ == "__main__":
    main()
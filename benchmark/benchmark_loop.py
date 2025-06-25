import time
import json
import numpy as np

def simulate_state():
    # Simulate sensor data for benchmarking
    return {
        "cpu_temp": float(np.random.normal(60, 5)),
        "core_freqs": [int(np.random.normal(3000, 100)) for _ in range(4)],
        "ppt": float(np.random.normal(40, 5)),
    }

def main():
    iterations = 100
    total_time = 0.0

    for i in range(iterations):
        state = simulate_state()
        t0 = time.perf_counter()
        # Simulate RL agent inference
        action = int(np.random.choice([-1, 0, 1]))
        t1 = time.perf_counter()
        total_time += (t1 - t0)
        if i % 10 == 0:
            print(f"Iter {i}: state={state} -> action={action}")

    print(f"Avg agent inference time: {total_time/iterations*1000:.3f} ms")

if __name__ == "__main__":
    main()
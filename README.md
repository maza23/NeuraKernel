# NeuraKernel: An AI-Driven Kernel Prototype for Dynamic Hardware Control

**Author:** Pablo Mazaeda  
**Version:** v0.2 Alpha  
**Date:** June 2025  

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/maza23/NeuraKernel)](https://github.com/maza23/NeuraKernel/releases)
[![License](https://img.shields.io/github/license/maza23/NeuraKernel)](./LICENSE)

---

> ⚠️ **Warning:** This project manipulates critical hardware parameters. Use at your own risk and only in controlled or test environments.

---

## Abstract

NeuraKernel is an experimental kernel prototype exploring AI-driven dynamic hardware control. Using reinforcement learning (RL), the system aims to optimize CPU frequency and power settings in real time, adapting to workload and context, and surpassing the limitations of traditional rule-based governors.

---

## 1. Introduction

Modern operating systems include CPU frequency governors based on predefined heuristics. While effective, these are not adaptive and may fail to achieve the best performance/energy balance. NeuraKernel investigates whether a kernel can delegate part of hardware control to an AI agent that learns optimal policies directly from sensors.

---

## 2. Related Work

Several projects explore AI-based resource management in datacenters and embedded systems:
1. H. Mao et al., “Resource Management with Deep Reinforcement Learning,” *Proc. CoRL*, 2016.  
2. F. Belletti et al., “Learning to Control Power: Reinforcement Learning for Microprocessor Power Management,” *arXiv preprint arXiv:2202.10478*, 2022.  
3. Y. Gao et al., “Power-Aware Resource Management Using Reinforcement Learning,” *Future Generation Computer Systems*, vol. 99, pp. 122–131, 2019.  
4. ZenStates-Linux: [https://github.com/r4m0n/ZenStates-Linux](https://github.com/r4m0n/ZenStates-Linux)  
5. Koala Self-Tuning Linux: [https://github.com/koala-project](https://github.com/koala-project)  

Unlike these, NeuraKernel focuses on low-level, real-time control from within a kernel-friendly loop, aiming to integrate AI decision-making into future OS/hypervisor designs.

---

## 3. System Architecture

```
┌──────────────┐     ┌───────────────┐    ┌──────────────┐    ┌──────────────┐
│ CPU Sensors  │ ─>  │ Rust AI Driver│ ─> │ RL Agent (Py)│ ─> │ HW Control   │
└──────────────┘     └───────────────┘    └──────────────┘    └──────────────┘
```

- The AI driver collects temperature, frequency, and power data from `/sys`.
- The RL agent reads the state as JSON, decides actions (+1 = increase freq), and the driver applies them using `cpufreq`.

---

## 4. Getting Started

### Prerequisites

- Python 3.8+
- Rust toolchain (`cargo`)
- PyTorch

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/maza23/NeuraKernel.git
   cd NeuraKernel
   ```

2. Install Python dependencies:
   ```bash
   pip install -r requirements.txt
   ```

3. Build the Rust driver:
   ```bash
   cd ai-driver
   cargo build --release
   cd ..
   ```

### Running

1. Start the AI driver:
   ```bash
   ./ai-driver/target/release/ai-driver
   ```

2. Run the RL agent:
   ```bash
   python rl_agent/main.py
   ```

---

## 5. Implementation (v0.2)

**AI Driver (Rust):**
- Reads CPU temperature, core frequencies, and power from sysfs.
- Exports JSON to `/tmp/neurakernel_sensors.json` every 100 ms.
- Accepts actions: -1 (down), 0 (keep), +1 (up) and applies frequency control.

**RL Agent (Python):**
- Uses a dummy PPO policy in PyTorch.
- Input: `[cpu_temp, core0..coreN_freqs, ppt]`
- Output: action ∈ {-1, 0, +1}

**Control Loop:**
- Runs at 10Hz.
- Can be extended to multi-core, continuous control, or multi-agent policy.

---

## 6. Usage Examples

**Sample driver log:**
```
{"cpu_temp": 67.5, "core_freqs": [3200, 3200, 3200, 3200], "ppt": 45.2}
Action received: +1 (Increase frequency)
New core_freqs: [3400, 3400, 3400, 3400]
```

**Sample RL agent decision:**
```
Input state: [67.5, 3200, 3200, 3200, 45.2]
Action taken: 0 (Keep)
```

---

## 7. Preliminary Results

- Stable real-time sensor readout and control at 10Hz loop.
- RL agent successfully reads state and makes dummy decisions.
- Ready for phase 3: real training with custom reward function (perf/Watt, thermal thresholds).

---

## 8. Future Work

- Support fine-grained power states using MSRs (Intel/AMD) or SMC (Apple Silicon).
- Add support for ARM and RISC-V.
- Integrate performance counters (IPC, cache miss, context switches).
- Extend to GPU/NPU units (heterogeneous control).
- Train real RL policies under stress benchmarks (e.g., Blender, Prime95).

---

## 9. Ethical and Safety Considerations

- Ensure thermal limits and voltage ceilings are always enforced.
- Log all agent decisions for traceability.
- Train policies in simulation or safe mode before deployment.
- Make AI decisions explainable (XRL).

---

## 10. Conclusion

NeuraKernel demonstrates that AI agents can participate in low-level hardware management by learning control policies directly from sensor data. While early-stage, the architecture is modular and extensible.

---

## 11. Contributing

Contributions are welcome! Please open an issue or pull request, and see `CONTRIBUTING.md` for details.

---

## 12. License

This project is licensed under the terms of the MIT license. See the LICENSE file for details.

---

## References

1. Mao, H., Alizadeh, M., Menache, I., & Kandula, S. (2016). *Resource Management with Deep Reinforcement Learning*. CoRL.  
2. Belletti, F., et al. (2022). *Learning to Control Power*. arXiv:2202.10478.  
3. Gao, Y., et al. (2019). *Power-Aware Resource Management*. FGCS, 99, 122-131.  
4. ZenStates-Linux. [https://github.com/r4m0n/ZenStates-Linux](https://github.com/r4m0n/ZenStates-Linux)  
5. Koala Project. [https://github.com/koala-project](https://github.com/koala-project)

---

## Appendix A: Architectures and Projects Co-Designed with AI

NeuraKernel draws inspiration from several recent projects at the intersection of hardware architecture and AI-based control:

- **MIT AutoPIM** – Co-optimizes memory and scheduling using RL agents.
- **Gemmini (UC Berkeley)** – RISC-V accelerator for matrix multiply, adaptable to learning-based instruction sets.
- **OpenPiton-AI (Princeton)** – Scalable manycore, AI-ready.
- **Eyeriss (MIT)** – Spatial AI accelerator, applies AI principles in layout and control.
- **DynaPIM (Meta)** – Combines PIM with learning-based reconfiguration.
- **ZenStates-Linux** and **Koala** – User-space projects that adjust hardware state, albeit with fixed logic.

These projects show that both AI-for-architecture and architecture-for-AI are viable research directions — and NeuraKernel could evolve into a testbed bridging both domains.

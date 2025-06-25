import pytest
import numpy as np
from ppo_agent import PPOAgent

def test_agent_action_range():
    agent = PPOAgent(state_dim=6)
    for _ in range(10):
        state = np.random.rand(6)
        action = agent.select_action(state)
        assert action in [-1, 0, 1], "Action should be -1, 0, or 1"
"""Pytest fixtures for OpenGolfCoach tests."""

import pytest


@pytest.fixture
def basic_shot():
    """Basic driver shot with all common parameters."""
    return {
        "ball_speed_meters_per_second": 70.0,
        "vertical_launch_angle_degrees": 12.5,
        "horizontal_launch_angle_degrees": -2.0,
        "total_spin_rpm": 2800.0,
        "spin_axis_degrees": 15.0,
    }


@pytest.fixture
def minimal_shot():
    """Minimal shot with only required fields."""
    return {
        "ball_speed_meters_per_second": 70.0,
        "vertical_launch_angle_degrees": 12.0,
    }


@pytest.fixture
def backspin_sidespin_shot():
    """Shot with backspin/sidespin instead of total spin."""
    return {
        "ball_speed_meters_per_second": 65.0,
        "vertical_launch_angle_degrees": 14.0,
        "horizontal_launch_angle_degrees": 1.5,
        "backspin_rpm": 3500.0,
        "sidespin_rpm": -800.0,
    }


@pytest.fixture
def straight_shot():
    """Shot with zero horizontal deviation for classification test."""
    return {
        "ball_speed_meters_per_second": 70.0,
        "vertical_launch_angle_degrees": 12.0,
        "horizontal_launch_angle_degrees": 0.0,
        "total_spin_rpm": 2500.0,
        "spin_axis_degrees": 0.0,
    }


@pytest.fixture
def us_customary_shot():
    """Shot with US customary units input."""
    return {
        "vertical_launch_angle_degrees": 12.0,
        "horizontal_launch_angle_degrees": 0.0,
        "total_spin_rpm": 2300.0,
        "spin_axis_degrees": 0.0,
        "us_customary_units": {"ball_speed_mph": 150.0},
    }

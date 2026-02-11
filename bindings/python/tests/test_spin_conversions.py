"""Tests for spin component calculations."""

import json
import math

import opengolfcoach


class TestSpinConversions:
    """Tests for spin component calculations."""

    def test_total_spin_to_components(self, basic_shot):
        """Should calculate backspin/sidespin from total spin and axis."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert "backspin_rpm" in ogc
        assert "sidespin_rpm" in ogc

        # Verify math: backspin = total * cos(axis), sidespin = total * sin(axis)
        expected_backspin = 2800.0 * math.cos(math.radians(15.0))
        expected_sidespin = 2800.0 * math.sin(math.radians(15.0))

        assert abs(ogc["backspin_rpm"] - expected_backspin) < 5.0
        assert abs(ogc["sidespin_rpm"] - expected_sidespin) < 5.0

    def test_components_to_total_spin(self, backspin_sidespin_shot):
        """Should calculate total spin and axis from components."""
        result_json = opengolfcoach.calculate_derived_values(
            json.dumps(backspin_sidespin_shot)
        )
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert "total_spin_rpm" in ogc
        assert "spin_axis_degrees" in ogc

        # Verify math: total = sqrt(bs^2 + ss^2)
        expected_total = math.sqrt(3500.0**2 + 800.0**2)
        assert abs(ogc["total_spin_rpm"] - expected_total) < 5.0

    def test_pure_backspin(self):
        """Zero spin axis should produce pure backspin."""
        shot = {
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.0,
            "total_spin_rpm": 3000.0,
            "spin_axis_degrees": 0.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert abs(ogc["backspin_rpm"] - 3000.0) < 1.0
        assert abs(ogc["sidespin_rpm"]) < 1.0

    def test_positive_spin_axis_produces_draw_sidespin(self):
        """Positive spin axis should produce positive sidespin (draw spin)."""
        shot = {
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.0,
            "total_spin_rpm": 2800.0,
            "spin_axis_degrees": 20.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert ogc["sidespin_rpm"] > 0

    def test_negative_spin_axis_produces_fade_sidespin(self):
        """Negative spin axis should produce negative sidespin (fade spin)."""
        shot = {
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.0,
            "total_spin_rpm": 2800.0,
            "spin_axis_degrees": -20.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert ogc["sidespin_rpm"] < 0

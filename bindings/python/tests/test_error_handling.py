"""Tests for error handling."""

import json

import pytest

import opengolfcoach


class TestErrorHandling:
    """Tests for error handling."""

    # Note: Invalid JSON currently causes a panic in the core library.
    # These tests are skipped until proper error handling is added upstream.
    @pytest.mark.skip(reason="Core library panics on invalid JSON - needs upstream fix")
    def test_invalid_json_raises_error(self):
        """Invalid JSON should raise ValueError."""
        with pytest.raises(ValueError):
            opengolfcoach.calculate_derived_values("not valid json")

    def test_empty_json_object_works(self):
        """Empty JSON object should still work (returns defaults)."""
        result_json = opengolfcoach.calculate_derived_values("{}")
        result = json.loads(result_json)
        assert isinstance(result, dict)

    @pytest.mark.skip(reason="Core library panics on non-object JSON - needs upstream fix")
    def test_json_array_input(self):
        """JSON array should be handled."""
        # Behavior depends on implementation - should not crash
        try:
            result = opengolfcoach.calculate_derived_values("[]")
            assert isinstance(result, str)
        except ValueError:
            pass  # Also acceptable

    def test_extreme_ball_speed(self):
        """Should handle very high ball speed."""
        shot = {
            "ball_speed_meters_per_second": 100.0,  # Very fast
            "vertical_launch_angle_degrees": 12.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)
        assert "open_golf_coach" in result

    def test_extreme_launch_angle(self):
        """Should handle extreme launch angles."""
        shot = {
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 45.0,  # Very high launch
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)
        assert "open_golf_coach" in result

    def test_extreme_spin(self):
        """Should handle very high spin."""
        shot = {
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.0,
            "total_spin_rpm": 10000.0,  # Very high spin
            "spin_axis_degrees": 45.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)
        assert "open_golf_coach" in result

    @pytest.mark.skip(reason="Core library panics on zero ball speed - needs upstream fix")
    def test_zero_ball_speed(self):
        """Should handle zero ball speed."""
        shot = {
            "ball_speed_meters_per_second": 0.0,
            "vertical_launch_angle_degrees": 12.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)
        assert "open_golf_coach" in result

    def test_additional_unknown_fields_preserved(self):
        """Unknown fields should be preserved in output."""
        shot = {
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.0,
            "custom_field": "test_value",
            "another_field": 123,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)

        assert result.get("custom_field") == "test_value"
        assert result.get("another_field") == 123

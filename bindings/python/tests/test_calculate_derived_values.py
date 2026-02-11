"""Tests for the main calculate_derived_values function."""

import json

import opengolfcoach


class TestCalculateDerivedValues:
    """Tests for the main calculate_derived_values function."""

    def test_returns_json_string(self, basic_shot):
        """Result should be a valid JSON string."""
        result = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        assert isinstance(result, str)
        parsed = json.loads(result)
        assert isinstance(parsed, dict)

    def test_preserves_input_fields(self, basic_shot):
        """Original input fields should be preserved."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        assert result["ball_speed_meters_per_second"] == 70.0
        assert result["vertical_launch_angle_degrees"] == 12.5
        assert result["horizontal_launch_angle_degrees"] == -2.0
        assert result["total_spin_rpm"] == 2800.0
        assert result["spin_axis_degrees"] == 15.0

    def test_adds_open_golf_coach_section(self, basic_shot):
        """Result should contain open_golf_coach section."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        assert "open_golf_coach" in result
        assert isinstance(result["open_golf_coach"], dict)

    def test_calculates_carry_distance(self, basic_shot):
        """Should calculate realistic carry distance."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        carry = result["open_golf_coach"]["carry_distance_meters"]
        # 70 m/s ball speed should produce ~200-250m carry
        assert 150.0 < carry < 280.0

    def test_calculates_total_distance(self, basic_shot):
        """Total distance should include roll."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        carry = result["open_golf_coach"]["carry_distance_meters"]
        total = result["open_golf_coach"]["total_distance_meters"]
        assert total >= carry

    def test_calculates_offline_distance(self, basic_shot):
        """Should calculate lateral deviation."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        offline = result["open_golf_coach"]["offline_distance_meters"]
        assert isinstance(offline, (int, float))

    def test_calculates_hang_time(self, basic_shot):
        """Should calculate realistic hang time."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        hang_time = result["open_golf_coach"]["hang_time_seconds"]
        # Typical driver hang time 4-9 seconds
        assert 3.0 < hang_time < 12.0

    def test_calculates_peak_height(self, basic_shot):
        """Should calculate peak height."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        peak = result["open_golf_coach"]["peak_height_meters"]
        # Typical driver apex 20-50m
        assert 10.0 < peak < 80.0

    def test_calculates_descent_angle(self, basic_shot):
        """Should calculate descent angle."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        descent = result["open_golf_coach"]["descent_angle_degrees"]
        # Typical descent angle 30-50 degrees
        assert 20.0 < descent < 70.0

    def test_minimal_input(self, minimal_shot):
        """Should work with minimal required inputs."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(minimal_shot))
        result = json.loads(result_json)

        assert "open_golf_coach" in result
        assert "carry_distance_meters" in result["open_golf_coach"]

    def test_calculates_club_metrics(self, basic_shot):
        """Should estimate clubhead speed and smash factor."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert "club_speed_meters_per_second" in ogc
        assert "smash_factor" in ogc
        # Smash factor typically 1.3-1.55
        assert 1.2 < ogc["smash_factor"] < 1.6

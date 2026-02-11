"""Integration tests matching example.py scenarios."""

import json

import opengolfcoach


class TestIntegration:
    """Integration tests matching example.py scenarios."""

    def test_example_1_basic_shot(self):
        """Reproduce Example 1 from example.py."""
        shot = {
            "ball_speed_meters_per_second": 70.0,
            "vertical_launch_angle_degrees": 12.5,
            "horizontal_launch_angle_degrees": -2.0,
            "total_spin_rpm": 2800.0,
            "spin_axis_degrees": 15.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)

        ogc = result["open_golf_coach"]
        assert ogc["carry_distance_meters"] > 0
        assert ogc["backspin_rpm"] > 0
        assert "shot_name" in ogc

    def test_example_2_backspin_sidespin(self):
        """Reproduce Example 2 from example.py."""
        shot = {
            "ball_speed_meters_per_second": 65.0,
            "vertical_launch_angle_degrees": 14.0,
            "horizontal_launch_angle_degrees": 1.5,
            "backspin_rpm": 3500.0,
            "sidespin_rpm": -800.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)

        ogc = result["open_golf_coach"]
        assert "total_spin_rpm" in ogc
        assert "spin_axis_degrees" in ogc

    def test_example_3_minimal_input(self):
        """Reproduce Example 3 from example.py."""
        shot = {
            "ball_speed_meters_per_second": 75.0,
            "vertical_launch_angle_degrees": 11.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)

        assert "open_golf_coach" in result

    def test_example_4_batch_processing(self):
        """Reproduce Example 4 batch processing."""
        shots = [
            {"ball_speed_meters_per_second": 60.0, "vertical_launch_angle_degrees": 15.0},
            {"ball_speed_meters_per_second": 70.0, "vertical_launch_angle_degrees": 12.0},
            {"ball_speed_meters_per_second": 80.0, "vertical_launch_angle_degrees": 10.0},
        ]

        results = []
        for shot in shots:
            result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
            results.append(json.loads(result_json))

        # Verify increasing ball speed produces increasing carry
        carries = [r["open_golf_coach"]["carry_distance_meters"] for r in results]
        assert carries[0] < carries[1] < carries[2]

    def test_shot_classification_straight(self, straight_shot):
        """Test shot classification for straight shot."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(straight_shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert ogc["shot_name"] == "Straight"
        assert "shot_rank" in ogc
        assert "shot_color_rgb" in ogc

    def test_shot_classification_has_all_fields(self, basic_shot):
        """Shot classification should include all expected fields."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert "shot_name" in ogc
        assert "shot_rank" in ogc
        assert "shot_color_rgb" in ogc

    def test_landing_position_vector(self, basic_shot):
        """Should include landing position as 3D vector."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert "landing_position" in ogc
        landing = ogc["landing_position"]
        assert "x" in landing
        assert "y" in landing
        assert "z" in landing

    def test_landing_velocity_vector(self, basic_shot):
        """Should include landing velocity as 3D vector."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        assert "landing_velocity" in ogc
        velocity = ogc["landing_velocity"]
        assert "x" in velocity
        assert "y" in velocity
        assert "z" in velocity

    def test_real_world_driver_shot(self):
        """Test with realistic PGA Tour average driver data."""
        # Approximate PGA Tour average driver stats
        shot = {
            "ball_speed_meters_per_second": 76.0,  # ~170 mph
            "vertical_launch_angle_degrees": 10.5,
            "horizontal_launch_angle_degrees": 0.0,
            "total_spin_rpm": 2500.0,
            "spin_axis_degrees": 0.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        # PGA average carry is ~275 yards = ~251 meters
        carry_yards = ogc["us_customary_units"]["carry_distance_yards"]
        assert 240 < carry_yards < 310

    def test_wedge_shot(self):
        """Test with typical wedge shot data."""
        shot = {
            "ball_speed_meters_per_second": 45.0,  # ~100 mph
            "vertical_launch_angle_degrees": 30.0,
            "horizontal_launch_angle_degrees": 0.0,
            "total_spin_rpm": 9000.0,
            "spin_axis_degrees": 0.0,
        }
        result_json = opengolfcoach.calculate_derived_values(json.dumps(shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]

        # Wedge shot should have high descent angle
        assert ogc["descent_angle_degrees"] > 40

        # High spin should be preserved
        assert ogc["backspin_rpm"] > 8000

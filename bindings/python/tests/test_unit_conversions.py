"""Tests for US customary unit conversions."""

import json

import opengolfcoach


class TestUSCustomaryUnits:
    """Tests for US customary unit conversions."""

    def test_outputs_us_customary_units(self, basic_shot):
        """Should include US customary conversions."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        us = result["open_golf_coach"]["us_customary_units"]
        assert "ball_speed_mph" in us
        assert "carry_distance_yards" in us

    def test_mph_conversion(self, basic_shot):
        """Ball speed conversion should be accurate."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        us = result["open_golf_coach"]["us_customary_units"]
        # 70 m/s = ~156.6 mph
        assert abs(us["ball_speed_mph"] - 156.6) < 1.0

    def test_yards_conversion(self, basic_shot):
        """Distance conversion should be accurate."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)
        ogc = result["open_golf_coach"]
        us = ogc["us_customary_units"]

        # 1 meter = 1.0936133 yards
        expected_yards = ogc["carry_distance_meters"] * 1.0936133
        assert abs(us["carry_distance_yards"] - expected_yards) < 0.5

    def test_total_distance_yards(self, basic_shot):
        """Total distance in yards should be calculated."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        us = result["open_golf_coach"]["us_customary_units"]
        assert "total_distance_yards" in us
        assert us["total_distance_yards"] > us["carry_distance_yards"]

    def test_offline_distance_yards(self, basic_shot):
        """Offline distance in yards should be calculated."""
        result_json = opengolfcoach.calculate_derived_values(json.dumps(basic_shot))
        result = json.loads(result_json)

        us = result["open_golf_coach"]["us_customary_units"]
        assert "offline_distance_yards" in us

    def test_accepts_us_customary_input(self, us_customary_shot):
        """Should accept US customary input and process correctly."""
        result_json = opengolfcoach.calculate_derived_values(
            json.dumps(us_customary_shot)
        )
        result = json.loads(result_json)

        # Should have calculated values
        assert "open_golf_coach" in result
        assert "carry_distance_meters" in result["open_golf_coach"]
        # 150 mph should produce reasonable carry
        carry = result["open_golf_coach"]["carry_distance_meters"]
        assert carry > 100.0

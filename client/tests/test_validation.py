"""
Unit tests for ATC validation module.

Tests check_readback_required() and other validation functions.
"""

import pytest
import sys
import os

# Add project path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from core.validation import (
    check_readback_required,
    validate_atc_response,
    is_within_scope,
)


class TestCheckReadbackRequired:
    """Tests for check_readback_required() function."""
    
    def test_hold_short_runway(self):
        """Hold short instructions require readback."""
        result = check_readback_required(
            "Taxi to runway 28 via Alpha, hold short of runway 10"
        )
        assert "Hold short runway 10" in result
    
    def test_hold_short_generic(self):
        """Generic hold short should still be caught."""
        result = check_readback_required("Hold short")
        assert "Hold short instruction" in result
    
    def test_takeoff_clearance_runway(self):
        """Takeoff clearance with runway requires readback."""
        result = check_readback_required(
            "Cessna 12345, runway 28, cleared for takeoff"
        )
        assert "Runway 28" in result
    
    def test_landing_clearance_runway(self):
        """Landing clearance with runway requires readback."""
        result = check_readback_required(
            "Cessna 12345, cleared to land runway 28L"
        )
        assert "Runway 28L" in result
    
    def test_cleared_for_option(self):
        """Cleared for the option requires readback."""
        result = check_readback_required(
            "Cessna 12345, cleared for the option runway 28"
        )
        assert "Runway 28" in result
    
    def test_frequency_contact(self):
        """Frequency changes require readback."""
        result = check_readback_required(
            "Contact Oakland Approach on 124.5"
        )
        assert "Frequency 124.5" in result
    
    def test_frequency_monitor(self):
        """Monitor instructions also require readback."""
        result = check_readback_required(
            "Monitor ATIS on 127.85"
        )
        assert "Frequency 127.85" in result
    
    def test_squawk_code(self):
        """Squawk codes require readback."""
        result = check_readback_required(
            "Cessna 12345, squawk 4521"
        )
        assert "Squawk 4521" in result
    
    def test_multiple_readbacks(self):
        """Complex instruction with multiple readback elements."""
        result = check_readback_required(
            "Taxi to runway 28 via Alpha, hold short of runway 10, "
            "squawk 4521, contact tower on 118.3"
        )
        assert len(result) >= 3
        assert any("Hold short" in r for r in result)
        assert any("Squawk" in r for r in result)
        assert any("Frequency" in r for r in result)
    
    def test_no_readback_traffic(self):
        """Traffic advisories don't require readback."""
        result = check_readback_required(
            "Traffic 2 o'clock, 3 miles, southbound"
        )
        assert len(result) == 0
    
    def test_no_readback_radar_contact(self):
        """Radar contact doesn't require readback."""
        result = check_readback_required(
            "Radar contact, 5 miles south of Oakland"
        )
        assert len(result) == 0


class TestValidateAtcResponse:
    """Tests for validate_atc_response() function."""
    
    def test_valid_response(self):
        """Normal ATC response passes validation."""
        result = validate_atc_response(
            "Cessna 12345, Sacramento Tower, runway 28, cleared for takeoff"
        )
        assert result.valid is True
        assert len(result.issues) == 0
    
    def test_empty_response(self):
        """Empty response fails validation."""
        result = validate_atc_response("")
        assert result.valid is False
        assert "Empty response" in result.issues
    
    def test_too_short(self):
        """Very short response fails validation."""
        result = validate_atc_response("Ok")
        assert result.valid is False
        assert any("too short" in i for i in result.issues)
    
    def test_invalid_squawk_with_8(self):
        """Squawk codes with 8 are invalid (octal)."""
        result = validate_atc_response("Squawk 8521")
        assert any("Invalid squawk" in i for i in result.issues)
    
    def test_invalid_squawk_with_9(self):
        """Squawk codes with 9 are invalid (octal)."""
        result = validate_atc_response("Squawk 4591")
        assert any("Invalid squawk" in i for i in result.issues)
    
    def test_valid_squawk(self):
        """Valid octal squawk codes pass."""
        result = validate_atc_response("Cessna 12345, squawk 4521")
        assert not any("squawk" in i.lower() for i in result.issues)
    
    def test_prohibited_phrase_taking_off(self):
        """Prohibited phrase 'taking off' triggers warning."""
        result = validate_atc_response(
            "Cessna 12345, we see you taking off runway 28"
        )
        assert any("Prohibited" in w for w in result.warnings)
    
    def test_prohibited_phrase_any_traffic(self):
        """'Any traffic please advise' triggers warning."""
        result = validate_atc_response(
            "Lincoln traffic, any traffic please advise"
        )
        assert any("Prohibited" in w or "Discouraged" in w for w in result.warnings)


class TestIsWithinScope:
    """Tests for is_within_scope() function."""
    
    def test_vfr_in_scope(self):
        """Normal VFR response is in scope."""
        in_scope, reason = is_within_scope(
            "Cessna 12345, squawk 4521, radar contact"
        )
        assert in_scope is True
    
    def test_ifr_approach_out_of_scope(self):
        """IFR approach is out of scope."""
        in_scope, reason = is_within_scope(
            "Cessna 12345, cleared ILS runway 28 approach"
        )
        assert in_scope is False
        assert "IFR" in reason
    
    def test_sid_out_of_scope(self):
        """SID routing is out of scope."""
        in_scope, reason = is_within_scope(
            "Cessna 12345, cleared SID departure"
        )
        assert in_scope is False
    
    def test_star_out_of_scope(self):
        """STAR routing is out of scope."""
        in_scope, reason = is_within_scope(
            "Cessna 12345, cleared STAR arrival"
        )
        assert in_scope is False


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

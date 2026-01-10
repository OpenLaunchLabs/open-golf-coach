use std::collections::HashMap;
use std::sync::OnceLock;

static RANK_COLORS: OnceLock<HashMap<String, String>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct ShotClassificationResult {
    pub shot_name: String,
    pub shot_rank: String,
    pub shot_color_rgb: String,
}

/// Direction of the shot based on horizontal launch angle
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Pull,     // HLA < -3.0°
    Straight, // -3.0° <= HLA <= 3.0°
    Push,     // HLA > 3.0°
}

/// Shape/curvature of the shot based on spin axis
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Hook,  // spin_axis < -12.0°
    Draw,  // -12.0° <= spin_axis < -3.0°
    None,  // -3.0° <= spin_axis <= 3.0°
    Fade,  // 3.0° < spin_axis <= 12.0°
    Slice, // spin_axis > 12.0°
}

impl Direction {
    fn from_hla(hla: f64) -> Self {
        if hla < -3.0 {
            Direction::Pull
        } else if hla > 3.0 {
            Direction::Push
        } else {
            Direction::Straight
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Direction::Pull => "Pull",
            Direction::Straight => "Straight",
            Direction::Push => "Push",
        }
    }
}

impl Shape {
    fn from_spin_axis(spin_axis: f64) -> Self {
        if spin_axis < -12.0 {
            Shape::Hook
        } else if spin_axis < -3.0 {
            Shape::Draw
        } else if spin_axis > 12.0 {
            Shape::Slice
        } else if spin_axis > 3.0 {
            Shape::Fade
        } else {
            Shape::None
        }
    }

    fn as_str(&self) -> Option<&'static str> {
        match self {
            Shape::Hook => Some("Hook"),
            Shape::Draw => Some("Draw"),
            Shape::None => None,
            Shape::Fade => Some("Fade"),
            Shape::Slice => Some("Slice"),
        }
    }
}

macro_rules! include_rank_colors {
    () => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/shot_classification/rank_colors.toml"
        ))
    };
}

/// Helper to create a special case shot result
fn special_shot(name: &str, rank: &str) -> ShotClassificationResult {
    ShotClassificationResult {
        shot_name: name.to_string(),
        shot_rank: rank.to_string(),
        shot_color_rgb: rank_color_for(rank),
    }
}

pub fn classify_shot(
    ball_speed_mps: f64,
    vertical_launch_angle_deg: f64,
    horizontal_launch_angle_deg: f64,
    _total_spin_rpm: f64,
    spin_axis_deg: f64,
) -> Option<ShotClassificationResult> {
    // Putt override: very low launch angle and slow ball speed
    if vertical_launch_angle_deg.abs() < 0.1 && ball_speed_mps < 15.0 {
        return Some(ShotClassificationResult {
            shot_name: "Putt".to_string(),
            shot_rank: String::new(),
            shot_color_rgb: "0x808080".to_string(),
        });
    }

    // Worm Burner: low launch angle with decent speed
    // VLA < 5 and ball_speed > 20
    if vertical_launch_angle_deg < 5.0 && ball_speed_mps > 20.0 {
        return Some(special_shot("Worm Burner", "E"));
    }

    // Right Shank: extreme right direction, high launch
    // HLA > 12, VLA > 12
    if horizontal_launch_angle_deg > 12.0 && vertical_launch_angle_deg > 12.0 {
        return Some(special_shot("Right Shank", "E"));
    }

    // Left Shank: extreme left direction, high launch
    // HLA < -12, VLA > 12
    if horizontal_launch_angle_deg < -12.0 && vertical_launch_angle_deg > 12.0 {
        return Some(special_shot("Left Shank", "E"));
    }

    // Duck Hook: extreme draw/hook that dives low and left
    // ball_speed > 30, VLA < 15, spin_axis < -25
    if ball_speed_mps > 30.0 && vertical_launch_angle_deg < 15.0 && spin_axis_deg < -25.0 {
        return Some(special_shot("Duck Hook", "E"));
    }

    // Banana Slice: extreme slice that balloons high and right
    // ball_speed > 30, VLA > 20, spin_axis > 25
    if ball_speed_mps > 30.0 && vertical_launch_angle_deg > 20.0 && spin_axis_deg > 25.0 {
        return Some(special_shot("Banana Slice", "E"));
    }

    // Baby shots: opposite signs with small magnitudes (both < 2.0)
    // Baby Push Draw: HLA > 0 (push) and spin_axis < 0 (draw), both small
    // Baby Pull Fade: HLA < 0 (pull) and spin_axis > 0 (fade), both small
    let hla_abs = horizontal_launch_angle_deg.abs();
    let spin_abs = spin_axis_deg.abs();
    if hla_abs < 2.0 && spin_abs < 2.0 {
        if horizontal_launch_angle_deg > 0.0 && spin_axis_deg < 0.0 {
            return Some(special_shot("Baby Push Draw", "S+"));
        } else if horizontal_launch_angle_deg < 0.0 && spin_axis_deg > 0.0 {
            return Some(special_shot("Baby Pull Fade", "S"));
        }
        // If both same sign or one is zero, fall through to normal classification
    }

    // Determine direction and shape
    let direction = Direction::from_hla(horizontal_launch_angle_deg);
    let shape = Shape::from_spin_axis(spin_axis_deg);

    // Build shot name
    let shot_name = match shape.as_str() {
        Some(shape_str) => format!("{} {}", direction.as_str(), shape_str),
        None => direction.as_str().to_string(),
    };

    // Assign rank based on shot type
    let shot_rank = get_shot_rank(direction, shape);
    let shot_color_rgb = rank_color_for(&shot_rank);

    Some(ShotClassificationResult {
        shot_name,
        shot_rank,
        shot_color_rgb,
    })
}

fn get_shot_rank(direction: Direction, shape: Shape) -> String {
    // Ranks focused toward beginners - push/fade shots imply better swing mechanics
    match (direction, shape) {
        // A: Shots with controlled curve
        (Direction::Straight, Shape::Draw) => "A".to_string(),
        (Direction::Straight, Shape::Fade) => "A".to_string(),
        (Direction::Push, Shape::Draw) => "A".to_string(),

        // B: No curve shots (two-way miss potential)
        (Direction::Straight, Shape::None) => "B".to_string(),
        (Direction::Pull, Shape::None) => "B".to_string(),
        (Direction::Push, Shape::None) => "B".to_string(),
        (Direction::Pull, Shape::Fade) => "B".to_string(),

        // C: Shots curving away from target
        (Direction::Pull, Shape::Draw) => "C".to_string(),
        (Direction::Push, Shape::Fade) => "C".to_string(),
        (Direction::Push, Shape::Hook) => "C".to_string(),
        (Direction::Pull, Shape::Slice) => "C".to_string(),

        // D: Extreme curves (hooks/slices)
        (Direction::Pull, Shape::Hook) => "D".to_string(),
        (Direction::Push, Shape::Slice) => "D".to_string(),
        (Direction::Straight, Shape::Hook) => "D".to_string(),
        (Direction::Straight, Shape::Slice) => "D".to_string(),
    }
}

fn parse_string(value: &str) -> String {
    value.trim().trim_matches('"').to_string()
}

fn normalize_color(input: &str) -> String {
    let trimmed = input.trim().trim_matches('"');
    let cleaned = trimmed.trim_start_matches("0x").trim_start_matches('#');
    format!("0x{}", cleaned.to_uppercase())
}

fn rank_color_for(rank: &str) -> String {
    let colors = RANK_COLORS.get_or_init(load_rank_colors);
    colors
        .get(rank)
        .cloned()
        .unwrap_or_else(|| "0xFFFFFF".to_string())
}

fn load_rank_colors() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let data = include_rank_colors!();
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with('[') {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            let rank_key = parse_string(key.trim());
            let color_raw = parse_string(value.trim());
            let color_value = normalize_color(&color_raw);
            map.insert(rank_key, color_value);
        }
    }

    if map.is_empty() {
        map.insert("S".to_string(), "0x00B3FF".to_string());
    }

    map
}

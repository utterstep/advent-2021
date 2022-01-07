use std::ops::{Bound, RangeBounds};

#[inline]
pub fn sum_up_to_n(n: i64) -> i64 {
    let n_abs = n.abs();

    (n_abs * (n_abs + 1)) / 2 * n.signum()
}

/// x = -1/2 sqrt(4 a^2 + 4 a - 8 b + 1) + a + 1/2 – root for positive values
/// x = 1/2 (sqrt(4 a^2 + 4 a - 8 b + 1) + 2 a + 1) – root for negative values
fn compute_root(initial_velocity: i64, target: i64) -> f64 {
    if target == 0 {
        return 0.;
    }

    let iv = initial_velocity as f64;
    let target = target as f64;
    let four_a_squared = 4. * iv * iv;
    let four_a = 4. * iv;
    let eight_b = 8. * target;
    let root = (four_a_squared + four_a - eight_b + 1.).sqrt();

    if target > 0. {
        -0.5 * root + iv + 0.5
    } else {
        0.5 * (root + 2. * iv + 1.)
    }
}

pub fn y_position(initial_velocity: i64, step_no: usize) -> i64 {
    let step_no = step_no as i64;

    if initial_velocity >= 0 {
        let step = if step_no > initial_velocity {
            step_no - 1
        } else {
            step_no
        };

        sum_up_to_n(initial_velocity) - sum_up_to_n((initial_velocity - step).abs())
    } else {
        sum_up_to_n(initial_velocity - step_no + 1) - sum_up_to_n(initial_velocity + 1)
    }
}

pub fn compute_hit<R1: RangeBounds<i64> + Clone, R2: RangeBounds<i64> + Clone>(
    x_range: &R1,
    y_range: &R2,
    x_velocity: i64,
    y_velocity: i64,
) -> Option<()> {
    let x_step_left = compute_x_hit(x_range, x_velocity, HorizontalBoundary::Left)?;

    let x_step_right = compute_x_hit(x_range, x_velocity, HorizontalBoundary::Right)?;

    let min_x_step = x_step_left.min(x_step_right);
    let max_x_step = x_step_left.max(x_step_right);

    let y_step_top = compute_y_hit(y_range, y_velocity, VerticalBoundary::Top)?;

    let y_step_bottom = compute_y_hit(y_range, y_velocity, VerticalBoundary::Bottom)?;

    let min_y_step = y_step_top.min(y_step_bottom);
    let max_y_step = y_step_top.max(y_step_bottom);

    (min_y_step <= max_x_step && max_y_step >= min_x_step).then(|| ())
}

#[derive(Debug)]
pub enum HorizontalBoundary {
    Left,
    Right,
}

fn compute_x_hit<R: RangeBounds<i64>>(
    target_range: &R,
    initial_velocity: i64,
    boundary: HorizontalBoundary,
) -> Option<usize> {
    let target_x = match boundary {
        HorizontalBoundary::Left => match target_range.start_bound() {
            Bound::Unbounded => return None,
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
        },
        HorizontalBoundary::Right => match target_range.end_bound() {
            Bound::Unbounded => return None,
            Bound::Included(end) => *end,
            Bound::Excluded(end) => *end - 1,
        },
    };

    let res = compute_root(initial_velocity, target_x);

    if res.is_nan() {
        return Some(usize::MAX);
    }

    let res = (match boundary {
        HorizontalBoundary::Left => res.ceil(),
        HorizontalBoundary::Right => res.floor(),
    }) as usize;

    if res > initial_velocity.abs() as usize {
        return None;
    }

    if target_range
        .contains(&(sum_up_to_n(initial_velocity) - sum_up_to_n(initial_velocity - res as i64)))
    {
        Some(res)
    } else {
        None
    }
}

#[derive(Debug)]
enum VerticalBoundary {
    Top,
    Bottom,
}

fn compute_y_hit<R: RangeBounds<i64>>(
    target_range: &R,
    initial_velocity: i64,
    boundary: VerticalBoundary,
) -> Option<usize> {
    let target_y = match boundary {
        VerticalBoundary::Top => match target_range.end_bound() {
            Bound::Unbounded => return None,
            Bound::Included(end) => *end,
            Bound::Excluded(end) => *end - 1,
        },
        VerticalBoundary::Bottom => match target_range.start_bound() {
            Bound::Unbounded => return None,
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
        },
    };

    let res = compute_root(initial_velocity, target_y);

    let res = (match boundary {
        // VerticalBoundary::Top => res.ceil(),
        // VerticalBoundary::Bottom => res.floor(),
        VerticalBoundary::Top => {
            if target_y < 0 {
                res.ceil()
            } else {
                res.floor()
            }
        }
        VerticalBoundary::Bottom => {
            if target_y < 0 {
                res.floor()
            } else {
                res.ceil()
            }
        }
    }) as i64;

    let res = res as usize;

    if target_range.contains(&y_position(initial_velocity, res)) {
        Some(res)
    } else {
        None
    }
}

pub fn compute_x_velocity<R: RangeBounds<i64>>(
    target_range: &R,
    boundary: HorizontalBoundary,
) -> Option<i64> {
    let target_x = match boundary {
        HorizontalBoundary::Left => match target_range.start_bound() {
            Bound::Unbounded => return None,
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
        },
        HorizontalBoundary::Right => match target_range.end_bound() {
            Bound::Unbounded => return None,
            Bound::Included(end) => *end,
            Bound::Excluded(end) => *end - 1,
        },
    };

    let x = target_x.abs();

    let res = 0.5 * (-1. + (1. + 8. * x as f64).sqrt());

    let candidate = (match boundary {
        HorizontalBoundary::Left => {
            if target_x > 0 {
                res.ceil()
            } else {
                res.floor()
            }
        }
        HorizontalBoundary::Right => {
            if target_x > 0 {
                res.floor()
            } else {
                res.ceil()
            }
        }
    }) as i64
        * target_x.signum();

    if target_range.contains(&sum_up_to_n(candidate)) {
        Some(candidate)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn trivial_y_pos_impl(start_velocity: i64, after_n_steps: usize) -> i64 {
        let mut velocity = start_velocity;
        let mut position = 0;

        for _ in 0..after_n_steps {
            position += velocity;
            velocity -= 1;
        }

        position
    }

    #[test]
    fn test_trivial_case() {
        assert_eq!(
            compute_x_velocity(&(0..=1), HorizontalBoundary::Left).unwrap(),
            0
        );
        assert_eq!(
            compute_x_velocity(&(0..=1), HorizontalBoundary::Right).unwrap(),
            1
        );
    }

    #[test]
    fn test_my_case() {
        assert_eq!(
            compute_x_velocity(&(85..=145), HorizontalBoundary::Left).unwrap(),
            13
        );
        assert_eq!(
            compute_x_velocity(&(85..=145), HorizontalBoundary::Right).unwrap(),
            16
        );
    }

    #[test]
    fn test_example_6_9() {
        assert_eq!(
            compute_y_hit(&(-10..=-5), 9, VerticalBoundary::Bottom).unwrap(),
            20
        );
        assert_eq!(
            compute_y_hit(&(-10..=-5), 9, VerticalBoundary::Top).unwrap(),
            20
        );

        assert_eq!(
            compute_x_hit(&(20..=30), 6, HorizontalBoundary::Left).unwrap(),
            5
        );
        assert_eq!(
            compute_x_hit(&(20..=30), 6, HorizontalBoundary::Right).unwrap(),
            usize::MAX
        );
    }

    #[test]
    fn test_example_25_neg9() {
        assert_eq!(
            compute_x_hit(&(20..=30), 25, HorizontalBoundary::Left).unwrap(),
            1
        );
        assert_eq!(
            compute_x_hit(&(20..=30), 25, HorizontalBoundary::Right).unwrap(),
            1
        );

        assert_eq!(
            compute_y_hit(&(-10..=-5), -9, VerticalBoundary::Bottom).unwrap(),
            1
        );
        assert_eq!(
            compute_y_hit(&(-10..=-5), -9, VerticalBoundary::Top).unwrap(),
            1
        );
    }

    #[test]
    fn test_example_8_0() {
        assert_eq!(
            compute_x_hit(&(20..=30), 8, HorizontalBoundary::Left).unwrap(),
            3
        );
        assert_eq!(
            compute_x_hit(&(20..=30), 8, HorizontalBoundary::Right).unwrap(),
            5
        );

        assert_eq!(
            compute_y_hit(&(-10..=-5), 0, VerticalBoundary::Bottom).unwrap(),
            5
        );
        assert_eq!(
            compute_y_hit(&(-10..=-5), 0, VerticalBoundary::Top).unwrap(),
            4
        );
    }

    #[test]
    fn test_some_examples() {
        let examples = [
            (23, -10),
            (25, -9),
            (27, -5),
            (29, -6),
            (22, -6),
            (21, -7),
            (9, 0),
            (27, -7),
            (24, -5),
            (25, -7),
            (26, -6),
            (25, -5),
            (6, 8),
            (11, -2),
            (20, -5),
            (29, -10),
            (6, 3),
            (28, -7),
        ];

        for (x_vel, y_vel) in examples {
            assert!(compute_hit(&(20..=30), &(-10..=-5), x_vel, y_vel).is_some());
        }
    }

    proptest! {
        #[test]
        fn sum_up_to_n_is_correct(n in 1..1000i64) {
            prop_assert_eq!(sum_up_to_n(n), (1..=n).sum());
        }

        #[test]
        fn x_speed_prediction(left in -1000..1000i64, size in 1..500i64) {
            let right = left + size;
            let range = left..=right;

            let left_prediction = compute_x_velocity(&range, HorizontalBoundary::Left);
            let right_prediction = compute_x_velocity(&range, HorizontalBoundary::Right);

            let (left_prediction, right_prediction) = match (left_prediction, right_prediction) {
                (Some(left_prediction), Some(right_prediction)) => (left_prediction, right_prediction),
                _ =>  {
                    return Err(TestCaseError::reject("skip case where target is reached midway"));
                }
            };

            let is_left_inside = range.contains(&sum_up_to_n(left_prediction));
            let is_right_inside = range.contains(&sum_up_to_n(right_prediction));

            let prev_left = left_prediction - 1;
            let next_right = right_prediction + 1;
            let is_left_optimal = (..left).contains(&sum_up_to_n(prev_left));
            let is_right_optimal = ((right + 1)..).contains(&sum_up_to_n(next_right));

            prop_assert!(is_left_inside && is_right_inside);
            prop_assert!(is_left_optimal && is_right_optimal);
        }

        #[test]
        fn y_pos_calc_correct(
            velocity in -100..100i64,
            steps_passed in 0..100usize
        ) {
            prop_assert_eq!(
                y_position(velocity, steps_passed),
                trivial_y_pos_impl(velocity, steps_passed)
            );
        }
    }
}

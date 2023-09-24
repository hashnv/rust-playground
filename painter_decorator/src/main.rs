fn main() {
    let room_height_meters = 2.5;
    let wall_a_length_meters = 8.0;
    let wall_b_length_meters = 10.0;

    let paint_coverage_coefficient: f64 = 10.0;

    let paint_can_cost_gbp = 12.50;
    let paint_can_cost_eur = 14.80;

    let total_surface_area: f64 =
        (2.0 * room_height_meters * (wall_a_length_meters + wall_b_length_meters)) as f64;
    let paint_cans_required = (total_surface_area / paint_coverage_coefficient).ceil();
    let total_paint_can_cost_gbp = paint_cans_required * paint_can_cost_gbp;
    let total_paint_can_cost_eur = paint_cans_required * paint_can_cost_eur;

    println!("total_surface_area: {} m^2", total_surface_area);
    println!("paint_cans_required: {} can(s)", paint_cans_required);
    println!(
        "total_paint_can_cost_gbp: {:.2} GBP",
        total_paint_can_cost_gbp,
    );
    println!(
        "total_paint_can_cost_eur: {:.2} EUR",
        total_paint_can_cost_eur,
    );
}

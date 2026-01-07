#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0)
var<uniform> u_color: vec4<f32>;

@group(1) @binding(1)
var<uniform> u_time: vec4<f32>;

@group(1) @binding(2)
var<uniform> u_percent: f32;

@group(1) @binding(3)
var<uniform> u_type: f32; // 0 = spin, 1 = percent

@group(1) @binding(4)
var<uniform> u_background_color: vec4<f32>;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let center = vec2(0.5, 0.5);
    let diff = uv - center;
    let dist = length(diff);

    let outer = 0.5;
    let inner = outer * 0.7; // Inner radius is 70% of outer radius
    let smoothness = 0.03;   // Fade amount for ring edges

    let fade_outer = smoothstep(outer - smoothness, outer, dist);
    let fade_inner = smoothstep(inner, inner + smoothness, dist);
    let ring_a = fade_inner * (1.0 - fade_outer); // Combined mask opacity

    if (ring_a < 0.01) {
        discard;
    }

    let pi = 3.14159265359;

    // --- Angle Calculation ---
    // raw_angle: -pi .. pi (Starts at Positive X-axis, counter-clockwise)
    let raw_angle = atan2(diff.y, diff.x);
    // angle: 0 .. 2pi (Starts at Positive X-axis, counter-clockwise)
    var angle = raw_angle + pi;


    // PERCENT MODE (u_type > 0.5) include background track
    if (u_type > 0.5) {

        // 1. ROTATE THE ANGLE
        // Shift angle by -pi/2 (90 degrees clockwise) so that 0 radians is at the TOP.
        var rotated_angle = angle - (pi * 0.5);

        // Handle wrap-around: if angle is negative, wrap it back to the [0, 2pi] range.
        if (rotated_angle < 0.0) {
            rotated_angle += 2.0 * pi;
        }

        // 2. DEFINE ARC LIMIT
        // Assumes u_percent is 0.0 to 100.0
        let limit = (u_percent / 100.0) * 2.0 * pi;

        // 3. COMPARISON and Color Selection

        // Define the final background and foreground colors (using ring_a for the mask)
        let foreground_final_color = vec4(u_color.rgb, u_color.a * ring_a);
        let background_final_color = vec4(u_background_color.rgb, u_background_color.a * ring_a);

        // Check if the pixel falls within the filled arc limit
        if (rotated_angle <= limit) {
            // Pixel is inside the filled arc
            return foreground_final_color;
        } else {
            // Pixel is outside the filled arc, render the background track color
            return background_final_color;
        }
        // Note: We no longer need 'discard' here because we are returning a color
        // for every pixel that passes the initial ring_a mask check.
    }

    //=========================
    // SPIN MODE (u_type <= 0.5)
    // Creates a fading tail that spins around the ring.
    //=========================

    let speed = -1.5;
    // Calculate the spinning position based on time and the pixel's angle.
    let spin_angle = fract((angle / (2.0 * pi)) + u_time.x * speed);

    let c_start = vec4(u_color.rgb, 1.0);// Bright part of the spinner
    let c_end = vec4(u_color.rgb, 0.2); // Faded tail/background part

    // Blend between bright and faded based on the spin position.
    let blended = mix(c_start, c_end, spin_angle);

    // Return the blended color, multiplied by the ring mask opacity.
    return vec4(blended.rgb, blended.a * ring_a);
}
